//! Android color scheme detection

use crate::theme::ColorScheme;

/// Detect the system color scheme on Android
pub fn detect_color_scheme() -> ColorScheme {
    // TODO: Use Configuration.uiMode
    // For now, default to light
    ColorScheme::Light
}
