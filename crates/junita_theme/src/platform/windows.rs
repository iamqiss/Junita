//! Windows color scheme detection

use crate::theme::ColorScheme;

/// Detect the system color scheme on Windows
pub fn detect_color_scheme() -> ColorScheme {
    // TODO: Use Windows.UI.ViewManagement.UISettings
    // For now, default to light
    ColorScheme::Light
}
