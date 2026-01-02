//! Skeleton component for loading placeholders
//!
//! A placeholder element that shows a shimmer/pulse effect while content loads.
//! Use with `motion()` for animated effects, or use as a static placeholder.
//!
//! # Example
//!
//! ```ignore
//! use blinc_cn::prelude::*;
//!
//! // Simple skeleton line
//! cn::skeleton().h(20.0).w(200.0)
//!
//! // Avatar skeleton
//! cn::skeleton().circle(48.0)
//!
//! // Card skeleton
//! div().col().gap(8.0)
//!     .child(cn::skeleton().h(200.0).w_full())  // Image
//!     .child(cn::skeleton().h(24.0).w(150.0))   // Title
//!     .child(cn::skeleton().h(16.0).w_full())   // Description line 1
//!     .child(cn::skeleton().h(16.0).w(80%))     // Description line 2
//!
//! // With pulse animation (requires motion)
//! motion()
//!     .pulse(1000)  // 1 second pulse animation
//!     .child(cn::skeleton().h(20.0))
//! ```

use std::ops::{Deref, DerefMut};

use blinc_layout::div::{Div, ElementBuilder, ElementTypeId};
use blinc_layout::prelude::*;
use blinc_theme::{ColorToken, RadiusToken, ThemeState};

/// Skeleton component for loading placeholders
pub struct Skeleton {
    inner: Div,
}

impl Skeleton {
    /// Create a new skeleton placeholder
    pub fn new() -> Self {
        let theme = ThemeState::get();

        // Use a muted background color for the skeleton
        let bg = theme.color(ColorToken::SurfaceElevated);
        let radius = theme.radius(RadiusToken::Default);

        let inner = div().bg(bg).rounded(radius);

        Self { inner }
    }

    /// Create a circular skeleton (for avatars, icons)
    pub fn circle(size: f32) -> Self {
        let theme = ThemeState::get();
        let bg = theme.color(ColorToken::SurfaceElevated);

        let inner = div()
            .bg(bg)
            .w(size)
            .h(size)
            .rounded(theme.radius(RadiusToken::Full));

        Self { inner }
    }

    /// Set width
    pub fn w(mut self, width: f32) -> Self {
        self.inner = self.inner.w(width);
        self
    }

    /// Set height
    pub fn h(mut self, height: f32) -> Self {
        self.inner = self.inner.h(height);
        self
    }

    /// Set full width
    pub fn w_full(mut self) -> Self {
        self.inner = self.inner.w_full();
        self
    }

    /// Set border radius
    pub fn rounded(mut self, radius: f32) -> Self {
        self.inner = self.inner.rounded(radius);
        self
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Skeleton {
    type Target = Div;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Skeleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ElementBuilder for Skeleton {
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

/// Create a skeleton placeholder
///
/// # Example
///
/// ```ignore
/// use blinc_cn::prelude::*;
///
/// // Text line skeleton
/// cn::skeleton().h(16.0).w(200.0)
///
/// // Avatar skeleton
/// cn::skeleton().circle(40.0)
/// ```
pub fn skeleton() -> Skeleton {
    Skeleton::new()
}

/// Create a circular skeleton
///
/// # Example
///
/// ```ignore
/// cn::skeleton_circle(48.0)  // 48px avatar placeholder
/// ```
pub fn skeleton_circle(size: f32) -> Skeleton {
    Skeleton::circle(size)
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
    fn test_skeleton_default() {
        init_theme();
        let _ = skeleton();
    }

    #[test]
    fn test_skeleton_sized() {
        init_theme();
        let _ = skeleton().h(20.0).w(200.0);
    }

    #[test]
    fn test_skeleton_circle() {
        init_theme();
        let _ = skeleton_circle(48.0);
    }
}
