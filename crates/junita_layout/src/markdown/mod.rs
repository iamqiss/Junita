//! Markdown rendering support for junita_layout
//!
//! This module provides markdown parsing and rendering to junita layout elements.
//! It uses pulldown-cmark for CommonMark + GFM extension parsing.
//!
//! # Example
//!
//! ```ignore
//! use junita_layout::prelude::*;
//! use junita_layout::markdown::markdown;
//!
//! // Simple markdown rendering
//! let content = markdown(r#"
//! # Hello World
//!
//! This is **bold** and *italic* text.
//!
//! - List item 1
//! - List item 2
//! "#);
//!
//! // Use in your layout
//! div().flex_col().child(content)
//! ```

mod config;
mod renderer;

pub use config::MarkdownConfig;
pub use renderer::{markdown, markdown_light, markdown_with_config, MarkdownRenderer};
