//! Spinner component for loading indicators
//!
//! An animated loading indicator. The animation is achieved via CSS-like
//! rotation or by using `motion()` for custom animations.
//!
//! # Example
//!
//! ```ignore
//! use blinc_cn::prelude::*;
//!
//! // Default spinner
//! cn::spinner()
//!
//! // Sized spinner
//! cn::spinner().size(SpinnerSize::Large)
//!
//! // Custom color
//! cn::spinner().color(Color::RED)
//! ```

use std::ops::{Deref, DerefMut};

use blinc_core::Color;
use blinc_layout::div::{Div, ElementBuilder, ElementTypeId};
use blinc_layout::prelude::*;
use blinc_theme::{ColorToken, ThemeState};

/// Spinner size variants
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    /// Small spinner (16px)
    Small,
    /// Medium spinner (24px)
    #[default]
    Medium,
    /// Large spinner (32px)
    Large,
}

impl SpinnerSize {
    fn size(&self) -> f32 {
        match self {
            SpinnerSize::Small => 16.0,
            SpinnerSize::Medium => 24.0,
            SpinnerSize::Large => 32.0,
        }
    }

    fn border_width(&self) -> f32 {
        match self {
            SpinnerSize::Small => 2.0,
            SpinnerSize::Medium => 2.5,
            SpinnerSize::Large => 3.0,
        }
    }
}

/// Spinner component for loading indicators
///
/// Displays a circular loading indicator. For animation, wrap with `motion()`
/// and use rotation animation, or use the native animation system.
pub struct Spinner {
    inner: Div,
}

impl Spinner {
    /// Create a new spinner
    pub fn new() -> Self {
        Self::with_size(SpinnerSize::default())
    }

    fn with_size(size: SpinnerSize) -> Self {
        let theme = ThemeState::get();

        let diameter = size.size();
        let border_width = size.border_width();
        let color = theme.color(ColorToken::Primary);
        let track_color = theme.color(ColorToken::Border);

        // Create a circular spinner
        // The visual appearance is a circle with a partial arc
        // For actual rotation animation, wrap with motion().rotate()
        let inner = div()
            .w(diameter)
            .h(diameter)
            .rounded(diameter / 2.0)
            .border(border_width, track_color);
        // Note: Actual spinning animation requires motion() or render-level animation

        Self { inner }
    }

    /// Set the spinner size
    pub fn size(self, size: SpinnerSize) -> Self {
        Self::with_size(size)
    }

    /// Set custom color for the spinner
    pub fn color(mut self, color: Color) -> Self {
        // Apply color as border
        self.inner = self.inner.border(2.5, color);
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Spinner {
    type Target = Div;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Spinner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ElementBuilder for Spinner {
    fn build(&self, tree: &mut blinc_layout::tree::LayoutTree) -> blinc_layout::tree::LayoutNodeId {
        self.inner.build(tree)
    }

    fn render_props(&self) -> blinc_layout::element::RenderProps {
        self.inner.render_props()
    }

    fn children_builders(&self) -> &[Box<dyn ElementBuilder>] {
        self.inner.children_builders()
    }

    fn event_handlers(&self) -> Option<&blinc_layout::event_handler::EventHandlers> {
        ElementBuilder::event_handlers(&self.inner)
    }

    fn layout_style(&self) -> Option<&taffy::Style> {
        ElementBuilder::layout_style(&self.inner)
    }

    fn element_type_id(&self) -> ElementTypeId {
        ElementBuilder::element_type_id(&self.inner)
    }
}

/// Create a spinner loading indicator
///
/// # Example
///
/// ```ignore
/// use blinc_cn::prelude::*;
///
/// // With rotation animation
/// motion()
///     .rotate_continuous(1000)  // 1 second per rotation
///     .child(cn::spinner())
/// ```
pub fn spinner() -> Spinner {
    Spinner::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_theme() {
        let _ = ThemeState::try_get().unwrap_or_else(|| {
            ThemeState::init_default();
            ThemeState::get()
        });
    }

    #[test]
    fn test_spinner_default() {
        init_theme();
        let _ = spinner();
    }

    #[test]
    fn test_spinner_sizes() {
        init_theme();
        let _ = spinner().size(SpinnerSize::Small);
        let _ = spinner().size(SpinnerSize::Medium);
        let _ = spinner().size(SpinnerSize::Large);
    }
}
