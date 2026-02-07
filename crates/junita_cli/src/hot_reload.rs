//! Hot reload server for development
//!
//! Watches for file changes, recompiles the application, and pushes updates
//! to a running client with state preservation.

use anyhow::Result;
use notify::{
    recommended_watcher, RecursiveMode, Watcher, Config, EventKind,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashSet;
use tracing::{info, warn, debug, error};
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use crate::compiler::JunitaCompiler;

/// Message sent from hot reload server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HotReloadMessage {
    /// Full rebuild required
    Rebuild {
        timestamp: u64,
    },
    /// Incremental update (specific files changed)
    Update {
        changed_files: Vec<PathBuf>,
        timestamp: u64,
    },
    /// State checkpoint for preservation
    SaveState,
    /// Restore to checkpoint after update
    RestoreState,
    /// Error occurred during compilation
    Error {
        message: String,
    },
}

/// Hot reload server configuration
#[derive(Clone)]
pub struct HotReloadConfig {
    /// Path to watch for changes
    pub watch_dir: PathBuf,
    /// Debounce delay in milliseconds
    pub debounce_ms: u64,
    /// File extensions to watch
    pub watch_extensions: Vec<String>,
    /// Paths to ignore
    pub ignore_patterns: Vec<String>,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            watch_dir: PathBuf::from("."),
            debounce_ms: 300,
            watch_extensions: vec![
                "junita".to_string(),
                "rs".to_string(),
                "toml".to_string(),
                "json".to_string(),
            ],
            ignore_patterns: vec![
                "target".to_string(),
                ".git".to_string(),
                "node_modules".to_string(),
                ".vscode".to_string(),
            ],
        }
    }
}

/// File watcher with debouncing
pub struct FileWatcher {
    tx: Arc<Mutex<broadcast::Sender<HotReloadMessage>>>,
    state: Arc<Mutex<WatcherState>>,
}

struct WatcherState {
    pending_changes: HashSet<PathBuf>,
    debounce_task: Option<tokio::task::JoinHandle<()>>,
    config: HotReloadConfig,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(config: HotReloadConfig) -> Result<(Self, broadcast::Receiver<HotReloadMessage>)> {
        let (tx, rx) = broadcast::channel(100);
        let tx = Arc::new(Mutex::new(tx));

        Ok((
            Self {
                tx,
                state: Arc::new(Mutex::new(WatcherState {
                    pending_changes: HashSet::new(),
                    debounce_task: None,
                    config,
                })),
            },
            rx,
        ))
    }

    /// Start watching the directory for changes
    pub async fn start(&self) -> Result<()> {
        let state = self.state.clone();
        let tx = self.tx.clone();
        let config = state.lock().unwrap().config.clone();

        info!("Starting file watcher for {:?}", config.watch_dir);

        let (watch_tx, mut watch_rx) = mpsc::channel();

        // Spawn file watcher on blocking thread
        let watch_dir = config.watch_dir.clone();
        tokio::task::spawn_blocking(move || {
            let mut watcher: Box<dyn Watcher> = match recommended_watcher(move |res: notify::Result<notify::Event>| {
                match res {
                    Ok(event) => {
                        if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                            for path in event.paths {
                                let _ = watch_tx.send(path);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("File watcher error: {}", e);
                    }
                }
            }) {
                Ok(w) => Box::new(w),
                Err(e) => {
                    error!("Failed to create file watcher: {}", e);
                    return;
                }
            };

            // Watch the directory
            if let Err(e) = watcher.watch(&watch_dir, RecursiveMode::Recursive) {
                error!("Failed to watch directory: {}", e);
                return;
            }

            // Keep watcher alive
            loop {
                std::thread::sleep(Duration::from_secs(1));
            }
        });

        // Process watch events
        while let Ok(file) = watch_rx.recv() {
            self.handle_event(file).await;
        }

        Ok(())
    }

    async fn handle_event(&self, file: PathBuf) {
        let mut st = self.state.lock().unwrap();
        let config = st.config.clone();

        // Check if file should be watched
        if !self.should_watch(&file, &config) {
            return;
        }

        debug!("File changed: {:?}", file);
        st.pending_changes.insert(file);

        // Cancel existing debounce task
        if let Some(task) = st.debounce_task.take() {
            task.abort();
        }

        // Schedule new debounce task
        let state = self.state.clone();
        let tx = self.tx.clone();
        let debounce_ms = config.debounce_ms;

        let task = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(debounce_ms)).await;

            let mut st = state.lock().unwrap();
            if !st.pending_changes.is_empty() {
                let changed: Vec<_> = st.pending_changes.drain().collect();
                info!("Files changed: {} file(s)", changed.len());

                let msg = HotReloadMessage::Update {
                    changed_files: changed,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64,
                };

                let _ = tx.lock().unwrap().send(msg);
            }
        });

        st.debounce_task = Some(task);
    }

    fn should_watch(&self, path: &Path, config: &HotReloadConfig) -> bool {
        // Check extension
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if !config.watch_extensions.iter().any(|e| e == ext) {
                return false;
            }
        } else {
            return false;
        }

        // Check ignore patterns
        let path_str = path.to_string_lossy();
        for pattern in &config.ignore_patterns {
            if path_str.contains(pattern) {
                return false;
            }
        }

        true
    }
}

/// Compilation trigger for hot reload with integrated Junita compiler
pub struct CompilationTrigger {
    project_path: PathBuf,
    target: String,
    compiler: Arc<Mutex<JunitaCompiler>>,
}

impl CompilationTrigger {
    pub fn new(project_path: PathBuf, target: String) -> Self {
        Self {
            project_path,
            target,
            compiler: Arc::new(Mutex::new(JunitaCompiler::new())),
        }
    }

    /// Trigger incremental recompilation using the Junita compiler
    pub async fn recompile(&self, changed_files: &[PathBuf]) -> Result<()> {
        info!(
            "Recompiling {} file(s) for target: {}",
            changed_files.len(),
            self.target
        );

        // Filter to only .junita and .rs files that might need compilation
        let junita_files: Vec<PathBuf> = changed_files
            .iter()
            .filter(|p| {
                let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                ext == "junita" || ext == "bl" || ext == "rs"
            })
            .cloned()
            .collect();

        if junita_files.is_empty() {
            debug!("No compilable files in change list");
            return Ok(());
        }

        // Compile using the Junita compiler (mock until real Zyntax available)
        let mut compiler = self.compiler.lock().unwrap();
        let artifacts = compiler.compile_incremental(&junita_files).await?;

        info!(
            "Compiled {} artifact(s) for hot reload",
            artifacts.len()
        );

        debug!("Recompilation complete");
        Ok(())
    }
}

/// Hot reload server that coordinates file watching and compilation
pub struct HotReloadServer {
    watcher: FileWatcher,
    compiler: CompilationTrigger,
    tx: Arc<Mutex<broadcast::Sender<HotReloadMessage>>>,
}

impl HotReloadServer {
    pub fn new(
        watch_dir: PathBuf,
        project_path: PathBuf,
        target: String,
    ) -> Result<(Self, broadcast::Receiver<HotReloadMessage>)> {
        let config = HotReloadConfig {
            watch_dir,
            ..Default::default()
        };

        let (watcher, rx) = FileWatcher::new(config)?;
        let compiler = CompilationTrigger::new(project_path, target);

        // Create a second receiver for the client
        let rx2 = rx.resubscribe();
        
        // Get the sender from the FileWatcher's watcher field
        let tx = watcher.tx.clone();

        Ok((
            Self {
                watcher,
                compiler,
                tx,
            },
            rx2,
        ))
    }

    /// Start the hot reload server
    pub async fn start(&self) -> Result<()> {
        info!("Hot reload server started");

        // Start file watcher
        self.watcher.start().await
    }

    /// Process update cycle
    pub async fn update_cycle(&self) -> Result<()> {
        let mut rx = self.tx.lock().unwrap().subscribe();

        while let Ok(msg) = rx.recv().await {
            match msg {
                HotReloadMessage::Update {
                    changed_files,
                    timestamp: _,
                } => {
                    // Trigger recompilation
                    if let Err(e) = self.compiler.recompile(&changed_files).await {
                        error!("Compilation failed: {}", e);
                        let err_msg = HotReloadMessage::Error {
                            message: e.to_string(),
                        };
                        let _ = self.tx.lock().unwrap().send(err_msg);
                    }
                }
                HotReloadMessage::Error { message } => {
                    error!("Hot reload error: {}", message);
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_watch() {
        let watcher = FileWatcher::new(HotReloadConfig::default())
            .map(|(w, _)| w)
            .unwrap();
        let config = HotReloadConfig::default();

        assert!(watcher.should_watch(Path::new("src/main.junita"), &config));
        assert!(watcher.should_watch(Path::new("src/lib.rs"), &config));
        assert!(!watcher.should_watch(Path::new("target/debug/app"), &config));
        assert!(!watcher.should_watch(Path::new(".git/config"), &config));
    }

    #[test]
    fn test_ignore_patterns() {
        let config = HotReloadConfig {
            ignore_patterns: vec!["node_modules".to_string()],
            ..Default::default()
        };

        let watcher = FileWatcher::new(config.clone())
            .map(|(w, _)| w)
            .unwrap();

        assert!(!watcher.should_watch(Path::new("node_modules/package/index.js"), &config));
        assert!(watcher.should_watch(Path::new("src/main.junita"), &config));
    }
}
