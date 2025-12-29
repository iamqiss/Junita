//! Linux color scheme detection

use crate::theme::ColorScheme;

/// Detect the system color scheme on Linux
pub fn detect_color_scheme() -> ColorScheme {
    // Try GTK settings first
    if let Some(scheme) = detect_gtk_color_scheme() {
        return scheme;
    }

    // Try XDG portal
    if let Some(scheme) = detect_xdg_color_scheme() {
        return scheme;
    }

    ColorScheme::Light
}

fn detect_gtk_color_scheme() -> Option<ColorScheme> {
    // Check GTK_THEME environment variable
    if let Ok(theme) = std::env::var("GTK_THEME") {
        let theme_lower = theme.to_lowercase();
        if theme_lower.contains("dark") {
            return Some(ColorScheme::Dark);
        }
    }

    // Check gsettings if available
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("dark") {
                return Some(ColorScheme::Dark);
            }
            if stdout.contains("light") || stdout.contains("default") {
                return Some(ColorScheme::Light);
            }
        }
    }

    None
}

fn detect_xdg_color_scheme() -> Option<ColorScheme> {
    // XDG Desktop Portal color scheme preference
    // Could use D-Bus to query org.freedesktop.portal.Settings
    // For now, return None
    None
}
