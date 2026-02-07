//! Debug server module for junita_recorder.
//!
//! This module provides a local socket server that allows external tools
//! (like junita_debugger) to connect and receive live recording data.
//!
//! Platform support:
//! - Unix (Linux/macOS): Unix domain sockets at `/tmp/junita/{app_name}.sock`
//! - Windows: Named pipes at `\\.\pipe\junita\{app_name}`

mod local;

pub use local::{
    start_local_server, start_local_server_named, ClientCommand, DebugServer, DebugServerConfig,
    ServerHandle, ServerMessage,
};
