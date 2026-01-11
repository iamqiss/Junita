//! Main application module for the debugger
//!
//! Handles window creation, event loop, and top-level state management.
//! Layout matches the Phase 12 plan with four main panels:
//! - Tree Panel (left): Element tree with diff
//! - Preview Panel (center): UI preview
//! - Inspector Panel (right): Element properties
//! - Timeline Panel (bottom): Event timeline with scrubber

use crate::panels::{
    InspectorPanel, PreviewConfig, PreviewPanel, TimelinePanel, TimelinePanelState, TreePanel,
    TreePanelState,
};
use crate::theme::{DebuggerColors, DebuggerTokens};
use anyhow::Result;
use blinc_layout::prelude::*;
use blinc_recorder::replay::{ReplayConfig, ReplayPlayer, ReplayState};
use blinc_recorder::{ElementSnapshot, RecordingExport, Timestamp, TreeSnapshot};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Application state
pub struct AppState {
    /// Loaded recording (if any)
    pub recording: Option<RecordingExport>,
    /// Replay player (if recording loaded)
    pub player: Option<Arc<Mutex<ReplayPlayer>>>,
    /// Current tree snapshot
    pub current_snapshot: Option<TreeSnapshot>,
    /// Selected element ID
    pub selected_element_id: Option<String>,
    /// Tree panel state
    pub tree_state: TreePanelState,
    /// Preview config
    pub preview_config: PreviewConfig,
    /// Timeline state
    pub timeline_state: TimelinePanelState,
    /// Connected to debug server
    pub connected: bool,
    /// Server address
    pub server_addr: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            recording: None,
            player: None,
            current_snapshot: None,
            selected_element_id: None,
            tree_state: TreePanelState::default(),
            preview_config: PreviewConfig::default(),
            timeline_state: TimelinePanelState::default(),
            connected: false,
            server_addr: None,
        }
    }
}

impl AppState {
    /// Load a recording from file
    pub fn load_recording(&mut self, path: &PathBuf) -> Result<()> {
        let contents = std::fs::read_to_string(path)?;
        let export: RecordingExport = serde_json::from_str(&contents)?;

        let player = ReplayPlayer::new(export.clone(), ReplayConfig::interactive());

        // Update timeline state with duration
        self.timeline_state.duration = player.duration();
        self.timeline_state.playback_state = ReplayState::Idle;

        // Load initial snapshot if available
        if let Some(snapshot) = export.snapshots.first() {
            self.current_snapshot = Some(snapshot.clone());
        }

        self.recording = Some(export);
        self.player = Some(Arc::new(Mutex::new(player)));

        log::info!("Loaded recording from {}", path.display());
        Ok(())
    }

    /// Get the selected element snapshot
    pub fn selected_element(&self) -> Option<&ElementSnapshot> {
        let snapshot = self.current_snapshot.as_ref()?;
        let id = self.selected_element_id.as_ref()?;
        snapshot.elements.get(id)
    }

    /// Get cursor position during replay
    pub fn cursor_position(&self) -> Option<(f32, f32)> {
        // TODO: Get from replay player's simulator
        None
    }
}

/// Build the main application UI with panel-based layout
pub fn build_ui(state: &AppState) -> impl ElementBuilder {
    div()
        .w_full()
        .h_full()
        .bg(DebuggerColors::BG_BASE)
        .flex_col()
        .child(
            // Main panel area (tree + preview + inspector)
            div()
                .flex_grow()
                .flex_row()
                .child(
                    // Tree Panel (left)
                    TreePanel::new(state.current_snapshot.as_ref(), &state.tree_state),
                )
                .child(
                    // Preview Panel (center)
                    PreviewPanel::new(
                        state.current_snapshot.as_ref(),
                        &state.preview_config,
                        state.cursor_position(),
                    ),
                )
                .child(
                    // Inspector Panel (right)
                    InspectorPanel::new(state.selected_element()),
                ),
        )
        .child(
            // Timeline Panel (bottom)
            TimelinePanel::new(
                state
                    .recording
                    .as_ref()
                    .map(|r| r.events.as_slice())
                    .unwrap_or(&[]),
                &state.timeline_state,
            ),
        )
}

/// Run the debugger application
pub fn run(
    _width: u32,
    _height: u32,
    _file: Option<PathBuf>,
    _connect: Option<String>,
) -> Result<()> {
    // TODO: Initialize windowed app with blinc_app
    // TODO: Set up event loop
    // TODO: Load recording if file provided
    // TODO: Connect to debug server if address provided

    log::info!("Debugger app scaffolding ready - implementation pending");

    // Placeholder - actual implementation will use blinc_app::run()
    Ok(())
}
