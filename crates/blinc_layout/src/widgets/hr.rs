//! Horizontal rule widget
//!
//! A simple horizontal separator line, equivalent to `<hr>` in HTML.
//!
//! # Example
//!
//! ```ignore
//! use blinc_layout::prelude::*;
//!
//! div()
//!     .child(p("Section 1"))
//!     .child(hr())
//!     .child(p("Section 2"))
//! ```

use blinc_core::Color;
use blinc_theme::{ColorToken, ThemeState};

use crate::div::{div, Div};

/// Configuration for horizontal rule styling
#[derive(Clone, Debug)]
pub struct HrConfig {
    /// Line color
    pub color: Color,
    /// Line thickness in pixels
    pub thickness: f32,
    /// Vertical margin above and below the line
    pub margin_y: f32,
}

impl Default for HrConfig {
    fn default() -> Self {
        let theme = ThemeState::get();
        Self {
            color: theme.color(ColorToken::Border),
            thickness: 1.0,
            margin_y: 16.0,
        }
    }
}

/// Create a horizontal rule (divider line)
///
/// Returns a styled Div that renders as a horizontal line.
///
/// # Example
///
/// ```ignore
/// hr()  // Default styling from theme
/// hr().my(8.0)  // Custom margin
/// ```
pub fn hr() -> Div {
    hr_with_config(HrConfig::default())
}

/// Create a horizontal rule with custom configuration
pub fn hr_with_config(config: HrConfig) -> Div {
    div()
        .w_full()
        .h(config.thickness)
        .bg(config.color)
        .my(config.margin_y)
}

/// Create a horizontal rule with custom color
pub fn hr_color(color: Color) -> Div {
    let mut config = HrConfig::default();
    config.color = color;
    hr_with_config(config)
}

/// Create a horizontal rule with custom thickness
pub fn hr_thick(thickness: f32) -> Div {
    let mut config = HrConfig::default();
    config.thickness = thickness;
    hr_with_config(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::div::ElementBuilder;
    use crate::tree::LayoutTree;

    fn init_theme() {
        let _ = ThemeState::try_get().unwrap_or_else(|| {
            ThemeState::init_default();
            ThemeState::get()
        });
    }

    #[test]
    fn test_hr_creates_div() {
        init_theme();
        let mut tree = LayoutTree::new();
        let rule = hr();
        rule.build(&mut tree);
        assert!(tree.len() > 0);
    }

    #[test]
    fn test_hr_with_custom_color() {
        init_theme();
        let mut tree = LayoutTree::new();
        let rule = hr_color(Color::RED);
        rule.build(&mut tree);
        assert!(tree.len() > 0);
    }
}
