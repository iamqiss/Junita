//! Linux Adwaita theme

use crate::theme::{ColorScheme, Theme, ThemeBundle};
use crate::themes::BlincTheme;
use crate::tokens::*;

/// Linux-native theme inspired by GNOME Adwaita
#[derive(Clone, Debug)]
pub struct LinuxTheme {
    inner: BlincTheme,
}

impl LinuxTheme {
    pub fn light() -> Self {
        // TODO: Customize with Adwaita colors
        Self {
            inner: BlincTheme::light(),
        }
    }

    pub fn dark() -> Self {
        // TODO: Customize with Adwaita colors
        Self {
            inner: BlincTheme::dark(),
        }
    }

    pub fn bundle() -> ThemeBundle {
        ThemeBundle::new("Linux", Self::light(), Self::dark())
    }
}

impl Theme for LinuxTheme {
    fn name(&self) -> &str {
        "Linux"
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
