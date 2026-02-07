//! macOS color scheme detection

use crate::theme::ColorScheme;

/// Detect the system color scheme on macOS
pub fn detect_color_scheme() -> ColorScheme {
    // Try reading from defaults
    // defaults read -g AppleInterfaceStyle returns "Dark" for dark mode
    // or exits with error for light mode
    if let Ok(output) = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().eq_ignore_ascii_case("dark") {
                return ColorScheme::Dark;
            }
        }
    }

    ColorScheme::Light
}
