//! Inspector Panel - Selected element properties
//!
//! Displays detailed information about the selected element:
//! - Element ID and type
//! - Bounds (x, y, width, height)
//! - Computed styles
//! - Event handlers
//! - State (for stateful elements)

use crate::theme::{DebuggerColors, DebuggerTokens};
use blinc_layout::prelude::*;
use blinc_recorder::ElementSnapshot;

/// Inspector panel component
pub struct InspectorPanel<'a> {
    selected: Option<&'a ElementSnapshot>,
}

impl<'a> InspectorPanel<'a> {
    pub fn new(selected: Option<&'a ElementSnapshot>) -> Self {
        Self { selected }
    }

    /// Build the inspector panel
    pub fn build(self) -> impl ElementBuilder {
        div()
            .w(DebuggerTokens::INSPECTOR_WIDTH)
            .h_full()
            .bg(DebuggerColors::BG_ELEVATED)
            .border_l(1.0)
            .border_color(DebuggerColors::BORDER_SUBTLE)
            .flex_col()
            .child(self.header())
            .child(self.content())
    }

    fn header(&self) -> impl ElementBuilder {
        div()
            .h(DebuggerTokens::HEADER_HEIGHT)
            .px(DebuggerTokens::SPACE_4)
            .border_b(1.0)
            .border_color(DebuggerColors::BORDER_SUBTLE)
            .flex_row()
            .items_center()
            .child(
                text("Inspector")
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_PRIMARY)
                    .weight(DebuggerTokens::FONT_WEIGHT_SEMIBOLD),
            )
    }

    fn content(&self) -> impl ElementBuilder {
        div()
            .flex_grow()
            .overflow_y_auto()
            .p(DebuggerTokens::SPACE_4)
            .child(if let Some(element) = self.selected {
                self.render_element_info(element)
            } else {
                self.render_empty_state()
            })
    }

    fn render_element_info(&self, element: &ElementSnapshot) -> impl ElementBuilder {
        div()
            .flex_col()
            .gap(DebuggerTokens::SPACE_4)
            .child(self.section("Element", vec![
                ("ID", element.id.as_str()),
                ("Type", "div"), // TODO: Get from element
            ]))
            .child(self.bounds_section(&element.bounds))
            .child(self.section("State", vec![
                ("Visible", if element.is_visible { "Yes" } else { "No" }),
                ("Focused", if element.is_focused { "Yes" } else { "No" }),
            ]))
    }

    fn section(&self, title: &str, properties: Vec<(&str, &str)>) -> impl ElementBuilder {
        div()
            .flex_col()
            .gap(DebuggerTokens::SPACE_2)
            .child(
                text(title)
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(DebuggerColors::TEXT_MUTED)
                    .weight(DebuggerTokens::FONT_WEIGHT_SEMIBOLD),
            )
            .child(
                div()
                    .flex_col()
                    .gap(DebuggerTokens::SPACE_1)
                    .children(properties.into_iter().map(|(key, value)| {
                        self.property_row(key, value)
                    })),
            )
    }

    fn bounds_section(&self, bounds: &blinc_recorder::capture::Rect) -> impl ElementBuilder {
        div()
            .flex_col()
            .gap(DebuggerTokens::SPACE_2)
            .child(
                text("Bounds")
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(DebuggerColors::TEXT_MUTED)
                    .weight(DebuggerTokens::FONT_WEIGHT_SEMIBOLD),
            )
            .child(
                div()
                    .flex_col()
                    .gap(DebuggerTokens::SPACE_1)
                    .child(self.property_row_value("X", bounds.x))
                    .child(self.property_row_value("Y", bounds.y))
                    .child(self.property_row_value("Width", bounds.width))
                    .child(self.property_row_value("Height", bounds.height)),
            )
    }

    fn property_row(&self, key: &str, value: &str) -> impl ElementBuilder {
        div()
            .flex_row()
            .justify_between()
            .child(
                text(key)
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_SECONDARY),
            )
            .child(
                text(value)
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_PRIMARY),
            )
    }

    fn property_row_value(&self, key: &str, value: f32) -> impl ElementBuilder {
        div()
            .flex_row()
            .justify_between()
            .child(
                text(key)
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_SECONDARY),
            )
            .child(
                text(format!("{:.1}", value))
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::PRIMARY),
            )
    }

    fn render_empty_state(&self) -> impl ElementBuilder {
        div()
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .child(
                text("Select an element")
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_MUTED),
            )
    }
}

impl<'a> ElementBuilder for InspectorPanel<'a> {
    fn build_element(self) -> blinc_layout::element::Element {
        self.build().build_element()
    }
}
