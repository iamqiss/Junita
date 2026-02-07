//! iOS theme

use crate::theme::{ColorScheme, Theme, ThemeBundle};
use crate::themes::JunitaTheme;
use crate::tokens::*;

/// iOS-native theme following Human Interface Guidelines
#[derive(Clone, Debug)]
pub struct IOSTheme {
    inner: JunitaTheme,
}

impl IOSTheme {
    pub fn light() -> Self {
        // TODO: Customize with iOS colors
        Self {
            inner: JunitaTheme::light(),
        }
    }

    pub fn dark() -> Self {
        // TODO: Customize with iOS colors
        Self {
            inner: JunitaTheme::dark(),
        }
    }

    pub fn bundle() -> ThemeBundle {
        ThemeBundle::new("iOS", Self::light(), Self::dark())
    }
}

impl Theme for IOSTheme {
    fn name(&self) -> &str {
        "iOS"
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
