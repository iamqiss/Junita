//! Windows Fluent Design theme

use crate::theme::{ColorScheme, Theme, ThemeBundle};
use crate::themes::BlincTheme;
use crate::tokens::*;

/// Windows-native theme inspired by Fluent Design System
#[derive(Clone, Debug)]
pub struct WindowsTheme {
    inner: BlincTheme,
}

impl WindowsTheme {
    pub fn light() -> Self {
        // TODO: Customize with Windows Fluent colors
        Self {
            inner: BlincTheme::light(),
        }
    }

    pub fn dark() -> Self {
        // TODO: Customize with Windows Fluent colors
        Self {
            inner: BlincTheme::dark(),
        }
    }

    pub fn bundle() -> ThemeBundle {
        ThemeBundle::new("Windows", Self::light(), Self::dark())
    }
}

impl Theme for WindowsTheme {
    fn name(&self) -> &str {
        "Windows"
    }

    fn color_scheme(&self) -> ColorScheme {
        self.inner.color_scheme()
    }

    fn colors(&self) -> &ColorTokens {
        self.inner.colors()
    }

    fn typography(&self) -> &TypographyTokens {
        self.inner.typography()
    }

    fn spacing(&self) -> &SpacingTokens {
        self.inner.spacing()
    }

    fn radii(&self) -> &RadiusTokens {
        self.inner.radii()
    }

    fn shadows(&self) -> &ShadowTokens {
        self.inner.shadows()
    }

    fn animations(&self) -> &AnimationTokens {
        self.inner.animations()
    }
}
