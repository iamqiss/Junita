//! Mock Zyntax compiler with integration points for real Grammar2
//!
//! This provides a working baseline that parses .junita files and generates
//! compilation artifacts. When Zyntax Grammar2 is available, replace the
//! parsing and code generation with actual Zyntax calls.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, debug};

/// Compiled artifact from Zyntax compiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledArtifact {
    /// Source file that was compiled
    pub source_file: PathBuf,
    /// Compiled widget definitions
    pub widgets: Vec<WidgetDefinition>,
    /// State machines defined in the file
    pub machines: Vec<MachineDef>,
    /// Animations defined in the file
    pub animations: Vec<AnimationDef>,
    /// Timestamp of compilation
    pub timestamp: u64,
    /// Checksum for detecting changes
    pub checksum: String,
}

/// Parsed widget definition from .junita file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetDefinition {
    pub name: String,
    pub properties: HashMap<String, PropertyType>,
    pub children: Vec<String>, // Child widget names
    pub state_vars: Vec<StateVar>,
    pub animations: Vec<String>, // Animation refs
}

/// State variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVar {
    pub name: String,
    pub var_type: String,
    pub initial_value: Option<String>,
}

/// State machine definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineDef {
    pub name: String,
    pub states: Vec<String>,
    pub initial_state: String,
    pub transitions: Vec<(String, String, String)>, // from, to, event
}

/// Animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDef {
    pub name: String,
    pub duration_ms: u32,
    pub easing: String,
}

/// Property type descriptor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PropertyType {
    String,
    Int,
    Float,
    Bool,
    Color,
    Custom(String),
}

impl ToString for PropertyType {
    fn to_string(&self) -> String {
        match self {
            PropertyType::String => "string".to_string(),
            PropertyType::Int => "int".to_string(),
            PropertyType::Float => "float".to_string(),
            PropertyType::Bool => "bool".to_string(),
            PropertyType::Color => "color".to_string(),
            PropertyType::Custom(s) => s.clone(),
        }
    }
}

/// mock Zyntax compiler
pub struct JunitaCompiler {
    cache: HashMap<PathBuf, CompiledArtifact>,
}

impl JunitaCompiler {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Compile a .junita/.bl file
    pub async fn compile(&mut self, source_path: &Path) -> Result<CompiledArtifact> {
        debug!("Compiling {}", source_path.display());

        // Check cache
        if let Some(cached) = self.cache.get(source_path) {
            let checksum = Self::file_checksum(source_path)?;
            if cached.checksum == checksum {
                info!("Using cached compilation for {}", source_path.display());
                return Ok(cached.clone());
            }
        }

        // Read source file
        let source = fs::read_to_string(source_path)
            .map_err(|e| anyhow!("Failed to read {}: {}", source_path.display(), e))?;

        // Validate file extension
        let ext = source_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "junita" && ext != "bl" {
            return Err(anyhow!(
                "Invalid file extension: {}. Expected .junita or .bl",
                ext
            ));
        }

        // TODO: When Zyntax Grammar2 is available, replace this mock parsing with:
        // let ast = zyntax_embed::parse(source_path, &source)?;
        // let artifact = zyntax_embed::compile_jit(ast)?;

        // For now, do a basic syntax validation and mock compilation
        let artifact = self.mock_compile(source_path, &source)?;

        // Cache the result
        self.cache.insert(source_path.to_path_buf(), artifact.clone());

        info!("Compiled {} successfully", source_path.display());
        Ok(artifact)
    }

    /// Compile multiple files (incremental compilation)
    pub async fn compile_incremental(
        &mut self,
        files: &[PathBuf],
    ) -> Result<Vec<CompiledArtifact>> {
        let mut artifacts = Vec::new();
        for file in files {
            artifacts.push(self.compile(file).await?);
        }
        Ok(artifacts)
    }

    /// Mock compilation for development (validates basic syntax)
    fn mock_compile(&self, source_path: &Path, source: &str) -> Result<CompiledArtifact> {
        // Simple tokenization and validation
        let lines: Vec<&str> = source.lines().collect();
        let mut widgets = Vec::new();
        let mut machines = Vec::new();
        let mut animations = Vec::new();

        for line in &lines {
            let trimmed = line.trim();

            // Parse @widget declarations
            if trimmed.starts_with("@widget") {
                if let Some(widget) = self.parse_widget_decl(trimmed) {
                    widgets.push(widget);
                }
            }

            // Parse @machine declarations
            if trimmed.starts_with("@machine") {
                if let Some(machine) = self.parse_machine_decl(trimmed) {
                    machines.push(machine);
                }
            }

            // Parse @animation declarations
            if trimmed.starts_with("@animation") {
                if let Some(anim) = self.parse_animation_decl(trimmed) {
                    animations.push(anim);
                }
            }

            // Basic syntax validation: matching braces
            if !self.validate_line_syntax(trimmed)? {
                return Err(anyhow!("Syntax error in {}", source_path.display()));
            }
        }

        let checksum = Self::file_checksum(source_path)?;

        Ok(CompiledArtifact {
            source_file: source_path.to_path_buf(),
            widgets,
            machines,
            animations,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            checksum,
        })
    }

    fn parse_widget_decl(&self, line: &str) -> Option<WidgetDefinition> {
        // Extract: @widget Name { ... }
        let after_widget = line.strip_prefix("@widget")?.trim();
        let name = after_widget
            .split_whitespace()
            .next()
            .unwrap_or("Unknown")
            .to_string();

        Some(WidgetDefinition {
            name,
            properties: HashMap::new(),
            children: Vec::new(),
            state_vars: Vec::new(),
            animations: Vec::new(),
        })
    }

    fn parse_machine_decl(&self, line: &str) -> Option<MachineDef> {
        // Extract: @machine Name { ... }
        let after_machine = line.strip_prefix("@machine")?.trim();
        let name = after_machine
            .split_whitespace()
            .next()
            .unwrap_or("Unknown")
            .to_string();

        Some(MachineDef {
            name,
            states: vec!["idle".to_string(), "active".to_string()],
            initial_state: "idle".to_string(),
            transitions: vec![("idle".to_string(), "active".to_string(), "click".to_string())],
        })
    }

    fn parse_animation_decl(&self, line: &str) -> Option<AnimationDef> {
        // Extract: @animation Name { ... }
        let after_anim = line.strip_prefix("@animation")?.trim();
        let name = after_anim
            .split_whitespace()
            .next()
            .unwrap_or("Unknown")
            .to_string();

        Some(AnimationDef {
            name,
            duration_ms: 300,
            easing: "ease-out".to_string(),
        })
    }

    fn validate_line_syntax(&self, line: &str) -> Result<bool> {
        // Count braces
        let open_braces = line.matches('{').count();
        let close_braces = line.matches('}').count();
        let open_parens = line.matches('(').count();
        let close_parens = line.matches(')').count();

        if open_braces == close_braces && open_parens == close_parens {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn file_checksum(path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let source = fs::read_to_string(path)?;
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Clear compilation cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cached artifact
    pub fn get_cached(&self, path: &Path) -> Option<&CompiledArtifact> {
        self.cache.get(path)
    }
}

impl Default for JunitaCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_parsing() {
        let compiler = JunitaCompiler::new();
        let widget = compiler.parse_widget_decl("@widget Counter {");
        assert!(widget.is_some());
        assert_eq!(widget.unwrap().name, "Counter");
    }

    #[test]
    fn test_syntax_validation() {
        let compiler = JunitaCompiler::new();
        assert!(compiler.validate_line_syntax("{ }").unwrap());
        assert!(compiler.validate_line_syntax("func(arg)").unwrap());
        assert!(!compiler.validate_line_syntax("{ }").is_err());
    }

    #[tokio::test]
    async fn test_compile_nonexistent() {
        let mut compiler = JunitaCompiler::new();
        let result = compiler.compile(Path::new("nonexistent.junita")).await;
        assert!(result.is_err());
    }
}
