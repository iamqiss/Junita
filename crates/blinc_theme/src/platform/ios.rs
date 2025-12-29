//! iOS color scheme detection

use crate::theme::ColorScheme;

/// Detect the system color scheme on iOS
pub fn detect_color_scheme() -> ColorScheme {
    // TODO: Use UITraitCollection.current.userInterfaceStyle
    // For now, default to light
    ColorScheme::Light
}
