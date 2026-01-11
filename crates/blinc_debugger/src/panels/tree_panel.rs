//! Tree Panel - Element tree with diff visualization
//!
//! Displays the element tree hierarchy with:
//! - Collapsible tree nodes
//! - Diff highlighting (added/removed/modified)
//! - Search/filter capability
//! - Selection state

use crate::theme::{DebuggerColors, DebuggerTokens};
use blinc_layout::prelude::*;
use blinc_recorder::TreeSnapshot;

/// State for the tree panel
pub struct TreePanelState {
    /// Currently selected element ID
    pub selected_id: Option<String>,
    /// Expanded node IDs
    pub expanded_ids: Vec<String>,
    /// Search/filter text
    pub filter_text: String,
}

impl Default for TreePanelState {
    fn default() -> Self {
        Self {
            selected_id: None,
            expanded_ids: Vec::new(),
            filter_text: String::new(),
        }
    }
}

/// Tree panel component
pub struct TreePanel<'a> {
    snapshot: Option<&'a TreeSnapshot>,
    state: &'a TreePanelState,
}

impl<'a> TreePanel<'a> {
    pub fn new(snapshot: Option<&'a TreeSnapshot>, state: &'a TreePanelState) -> Self {
        Self { snapshot, state }
    }

    /// Build the tree panel
    pub fn build(self) -> impl ElementBuilder {
        div()
            .w(DebuggerTokens::TREE_PANEL_WIDTH)
            .h_full()
            .bg(DebuggerColors::BG_ELEVATED)
            .border_r(1.0)
            .border_color(DebuggerColors::BORDER_SUBTLE)
            .flex_col()
            .child(self.header())
            .child(self.search_bar())
            .child(self.tree_content())
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
                text("Element Tree")
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_PRIMARY)
                    .weight(DebuggerTokens::FONT_WEIGHT_SEMIBOLD),
            )
    }

    fn search_bar(&self) -> impl ElementBuilder {
        // TODO: Use blinc_cn::input when integrated
        div()
            .px(DebuggerTokens::SPACE_3)
            .py(DebuggerTokens::SPACE_2)
            .child(
                div()
                    .w_full()
                    .h(32.0)
                    .px(DebuggerTokens::SPACE_3)
                    .bg(DebuggerColors::BG_SURFACE)
                    .rounded(DebuggerTokens::RADIUS_MD)
                    .border(1.0)
                    .border_color(DebuggerColors::BORDER_DEFAULT)
                    .items_center()
                    .child(
                        text("Search elements...")
                            .size(DebuggerTokens::FONT_SIZE_SM)
                            .color(DebuggerColors::TEXT_MUTED),
                    ),
            )
    }

    fn tree_content(&self) -> impl ElementBuilder {
        // TODO: Implement actual tree rendering from snapshot
        div()
            .flex_grow()
            .overflow_y_auto()
            .p(DebuggerTokens::SPACE_2)
            .child(if self.snapshot.is_some() {
                self.render_tree_placeholder()
            } else {
                self.render_empty_state()
            })
    }

    fn render_tree_placeholder(&self) -> impl ElementBuilder {
        // Placeholder tree nodes
        div()
            .flex_col()
            .gap(DebuggerTokens::SPACE_1)
            .child(self.tree_node("root", 0, false))
            .child(self.tree_node("header", 1, false))
            .child(self.tree_node("main", 1, true))
            .child(self.tree_node("sidebar", 2, false))
            .child(self.tree_node("content", 2, false))
            .child(self.tree_node("footer", 1, false))
    }

    fn tree_node(&self, id: &str, depth: usize, is_selected: bool) -> impl ElementBuilder {
        let indent = depth as f32 * DebuggerTokens::SPACE_4;
        let bg = if is_selected {
            DebuggerColors::PRIMARY.with_alpha(0.2)
        } else {
            DebuggerColors::BG_ELEVATED
        };

        div()
            .w_full()
            .h(24.0)
            .pl(indent)
            .pr(DebuggerTokens::SPACE_2)
            .bg(bg)
            .rounded(DebuggerTokens::RADIUS_SM)
            .flex_row()
            .items_center()
            .gap(DebuggerTokens::SPACE_2)
            .cursor_pointer()
            .child(
                // Expand/collapse icon
                text("\u{25B6}") // Triangle
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(DebuggerColors::TEXT_MUTED),
            )
            .child(
                text(id)
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(if is_selected {
                        DebuggerColors::PRIMARY
                    } else {
                        DebuggerColors::TEXT_SECONDARY
                    }),
            )
    }

    fn render_empty_state(&self) -> impl ElementBuilder {
        div()
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .child(
                text("No recording loaded")
                    .size(DebuggerTokens::FONT_SIZE_SM)
                    .color(DebuggerColors::TEXT_MUTED),
            )
    }
}

impl<'a> ElementBuilder for TreePanel<'a> {
    fn build_element(self) -> blinc_layout::element::Element {
        self.build().build_element()
    }
}
