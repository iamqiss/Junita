//! Preview Panel - Live/recorded UI preview
//!
//! Displays:
//! - Rendered UI from recording/live session
//! - Debug overlay (element bounds, hit areas)
//! - Cursor position indicator during replay

use crate::theme::{DebuggerColors, DebuggerTokens};
use blinc_core::Color;
use blinc_layout::prelude::*;
use blinc_recorder::TreeSnapshot;

/// Preview panel configuration
pub struct PreviewConfig {
    /// Show element bounds overlay
    pub show_bounds: bool,
    /// Show cursor during replay
    pub show_cursor: bool,
    /// Show hover highlights
    pub show_hover: bool,
    /// Zoom level (1.0 = 100%)
    pub zoom: f32,
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            show_bounds: false,
            show_cursor: true,
            show_hover: true,
            zoom: 1.0,
        }
    }
}

/// Preview panel component
pub struct PreviewPanel<'a> {
    snapshot: Option<&'a TreeSnapshot>,
    config: &'a PreviewConfig,
    cursor_position: Option<(f32, f32)>,
}

impl<'a> PreviewPanel<'a> {
    pub fn new(
        snapshot: Option<&'a TreeSnapshot>,
        config: &'a PreviewConfig,
        cursor_position: Option<(f32, f32)>,
    ) -> Self {
        Self {
            snapshot,
            config,
            cursor_position,
        }
    }

    /// Build the preview panel
    pub fn build(self) -> impl ElementBuilder {
        div()
            .flex_grow()
            .h_full()
            .bg(DebuggerColors::BG_BASE)
            .flex_col()
            .child(self.toolbar())
            .child(self.preview_area())
    }

    fn toolbar(&self) -> impl ElementBuilder {
        div()
            .h(DebuggerTokens::HEADER_HEIGHT)
            .px(DebuggerTokens::SPACE_4)
            .bg(DebuggerColors::BG_ELEVATED)
            .border_b(1.0)
            .border_color(DebuggerColors::BORDER_SUBTLE)
            .flex_row()
            .items_center()
            .justify_between()
            .child(
                text("Preview")
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_PRIMARY)
                    .weight(DebuggerTokens::FONT_WEIGHT_SEMIBOLD),
            )
            .child(
                // Toolbar buttons
                div()
                    .flex_row()
                    .gap(DebuggerTokens::SPACE_2)
                    .child(self.toolbar_button("Bounds", self.config.show_bounds))
                    .child(self.toolbar_button("Cursor", self.config.show_cursor))
                    .child(self.zoom_control()),
            )
    }

    fn toolbar_button(&self, label: &str, is_active: bool) -> impl ElementBuilder {
        let bg = if is_active {
            DebuggerColors::PRIMARY.with_alpha(0.2)
        } else {
            DebuggerColors::BG_SURFACE
        };
        let text_color = if is_active {
            DebuggerColors::PRIMARY
        } else {
            DebuggerColors::TEXT_SECONDARY
        };

        div()
            .px(DebuggerTokens::SPACE_3)
            .py(DebuggerTokens::SPACE_1)
            .bg(bg)
            .rounded(DebuggerTokens::RADIUS_SM)
            .cursor_pointer()
            .child(
                text(label)
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(text_color),
            )
    }

    fn zoom_control(&self) -> impl ElementBuilder {
        div()
            .flex_row()
            .items_center()
            .gap(DebuggerTokens::SPACE_2)
            .px(DebuggerTokens::SPACE_2)
            .child(
                text(format!("{}%", (self.config.zoom * 100.0) as i32))
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(DebuggerColors::TEXT_SECONDARY),
            )
    }

    fn preview_area(&self) -> impl ElementBuilder {
        div()
            .flex_grow()
            .overflow_clip()
            .items_center()
            .justify_center()
            .p(DebuggerTokens::SPACE_6)
            .child(if self.snapshot.is_some() {
                self.render_preview()
            } else {
                self.render_empty_state()
            })
    }

    fn render_preview(&self) -> impl ElementBuilder {
        // TODO: Render actual UI from snapshot using headless rendering
        // For now, show a placeholder
        div()
            .w(800.0 * self.config.zoom)
            .h(600.0 * self.config.zoom)
            .bg(DebuggerColors::BG_SURFACE)
            .rounded(DebuggerTokens::RADIUS_LG)
            .border(1.0)
            .border_color(DebuggerColors::BORDER_DEFAULT)
            .items_center()
            .justify_center()
            .relative()
            .child(
                text("UI Preview")
                    .size(DebuggerTokens::FONT_SIZE_LG)
                    .color(DebuggerColors::TEXT_MUTED),
            )
            .child_if(
                self.config.show_cursor && self.cursor_position.is_some(),
                || self.render_cursor(),
            )
    }

    fn render_cursor(&self) -> impl ElementBuilder {
        let (x, y) = self.cursor_position.unwrap_or((0.0, 0.0));

        div()
            .absolute()
            .left(x * self.config.zoom)
            .top(y * self.config.zoom)
            .w(12.0)
            .h(12.0)
            .rounded_full()
            .bg(DebuggerColors::PRIMARY)
            .border(2.0)
            .border_color(Color::WHITE)
    }

    fn render_empty_state(&self) -> impl ElementBuilder {
        div()
            .w(400.0)
            .flex_col()
            .items_center()
            .gap(DebuggerTokens::SPACE_4)
            .child(
                text("No Preview Available")
                    .size(DebuggerTokens::FONT_SIZE_LG)
                    .color(DebuggerColors::TEXT_PRIMARY),
            )
            .child(
                text("Load a recording or connect to a running app")
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_MUTED),
            )
    }
}

impl<'a> ElementBuilder for PreviewPanel<'a> {
    fn build_element(self) -> blinc_layout::element::Element {
        self.build().build_element()
    }
}
