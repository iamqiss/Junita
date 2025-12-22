//! Layout system tests
//!
//! Tests for the GPUI-style layout builder API powered by Taffy flexbox.

use crate::runner::TestSuite;
use blinc_core::{Color, DrawContext};
use blinc_layout::prelude::*;

/// Create the layout test suite
pub fn suite() -> TestSuite {
    let mut suite = TestSuite::new("layout");

    // Basic flex row layout
    suite.add("flex_row_basic", |ctx| {
        let ui = div()
            .w(400.0)
            .h(100.0)
            .flex_row()
            .gap_px(10.0)
            .bg(Color::rgba(0.2, 0.2, 0.2, 1.0))
            .child(div().w(80.0).h(80.0).bg(Color::RED))
            .child(div().w(80.0).h(80.0).bg(Color::GREEN))
            .child(div().w(80.0).h(80.0).bg(Color::BLUE));

        let mut tree = RenderTree::from_element(&ui);
        tree.compute_layout(800.0, 600.0);
        tree.render(ctx.ctx());
    });

    // Flex column with gap
    suite.add("flex_col_with_gap", |ctx| {
        let ui = div()
            .w(100.0)
            .h(400.0)
            .flex_col()
            .gap_px(15.0)
            .bg(Color::rgba(0.15, 0.15, 0.2, 1.0))
            .child(div().w_full().h(60.0).bg(Color::rgba(1.0, 0.3, 0.3, 1.0)))
            .child(div().w_full().h(60.0).bg(Color::rgba(0.3, 1.0, 0.3, 1.0)))
            .child(div().w_full().h(60.0).bg(Color::rgba(0.3, 0.3, 1.0, 1.0)))
            .child(div().w_full().h(60.0).bg(Color::rgba(1.0, 1.0, 0.3, 1.0)));

        let mut tree = RenderTree::from_element(&ui);
        tree.compute_layout(800.0, 600.0);
        tree.render(ctx.ctx());
    });

    // Flex grow distribution
    suite.add("flex_grow", |ctx| {
        let ui = div()
            .w(400.0)
            .h(100.0)
            .flex_row()
            .bg(Color::rgba(0.1, 0.1, 0.15, 1.0))
            .child(div().w(60.0).h(100.0).bg(Color::RED))
            .child(div().flex_grow().h(100.0).bg(Color::GREEN))
            .child(div().w(60.0).h(100.0).bg(Color::BLUE));

        let mut tree = RenderTree::from_element(&ui);
        tree.compute_layout(800.0, 600.0);
        tree.render(ctx.ctx());
    });

    // Nested layout
    suite.add("nested_layout", |ctx| {
        let ui = div()
            .w(300.0)
            .h(300.0)
            .flex_col()
            .gap_px(10.0)
            .bg(Color::rgba(0.2, 0.2, 0.25, 1.0))
            .child(
                div()
                    .w_full()
                    .h(80.0)
                    .flex_row()
                    .gap_px(10.0)
                    .child(div().w(80.0).h(80.0).rounded(8.0).bg(Color::RED))
                    .child(div().flex_grow().h(80.0).rounded(8.0).bg(Color::ORANGE)),
            )
            .child(
                div()
                    .w_full()
                    .flex_grow()
                    .flex_row()
                    .gap_px(10.0)
                    .child(div().w(100.0).h_full().rounded(8.0).bg(Color::YELLOW))
                    .child(div().flex_grow().h_full().rounded(8.0).bg(Color::GREEN)),
            )
            .child(div().w_full().h(50.0).rounded(8.0).bg(Color::CYAN));

        let mut tree = RenderTree::from_element(&ui);
        tree.compute_layout(800.0, 600.0);
        tree.render(ctx.ctx());
    });

    // Justify content variations
    suite.add("justify_content", |ctx| {
        let c = ctx.ctx();

        // justify-start (default)
        let row1 = div()
            .w(350.0)
            .h(60.0)
            .flex_row()
            .justify_start()
            .bg(Color::rgba(0.3, 0.3, 0.35, 1.0))
            .child(div().w(50.0).h(50.0).bg(Color::RED))
            .child(div().w(50.0).h(50.0).bg(Color::GREEN))
            .child(div().w(50.0).h(50.0).bg(Color::BLUE));

        let mut tree1 = RenderTree::from_element(&row1);
        tree1.compute_layout(800.0, 600.0);
        c.push_transform(blinc_core::Transform::translate(25.0, 20.0));
        tree1.render(c);
        c.pop_transform();

        // justify-center
        let row2 = div()
            .w(350.0)
            .h(60.0)
            .flex_row()
            .justify_center()
            .bg(Color::rgba(0.3, 0.3, 0.35, 1.0))
            .child(div().w(50.0).h(50.0).bg(Color::RED))
            .child(div().w(50.0).h(50.0).bg(Color::GREEN))
            .child(div().w(50.0).h(50.0).bg(Color::BLUE));

        let mut tree2 = RenderTree::from_element(&row2);
        tree2.compute_layout(800.0, 600.0);
        c.push_transform(blinc_core::Transform::translate(25.0, 100.0));
        tree2.render(c);
        c.pop_transform();

        // justify-end
        let row3 = div()
            .w(350.0)
            .h(60.0)
            .flex_row()
            .justify_end()
            .bg(Color::rgba(0.3, 0.3, 0.35, 1.0))
            .child(div().w(50.0).h(50.0).bg(Color::RED))
            .child(div().w(50.0).h(50.0).bg(Color::GREEN))
            .child(div().w(50.0).h(50.0).bg(Color::BLUE));

        let mut tree3 = RenderTree::from_element(&row3);
        tree3.compute_layout(800.0, 600.0);
        c.push_transform(blinc_core::Transform::translate(25.0, 180.0));
        tree3.render(c);
        c.pop_transform();

        // justify-between
        let row4 = div()
            .w(350.0)
            .h(60.0)
            .flex_row()
            .justify_between()
            .bg(Color::rgba(0.3, 0.3, 0.35, 1.0))
            .child(div().w(50.0).h(50.0).bg(Color::RED))
            .child(div().w(50.0).h(50.0).bg(Color::GREEN))
            .child(div().w(50.0).h(50.0).bg(Color::BLUE));

        let mut tree4 = RenderTree::from_element(&row4);
        tree4.compute_layout(800.0, 600.0);
        c.push_transform(blinc_core::Transform::translate(25.0, 260.0));
        tree4.render(c);
        c.pop_transform();
    });

    // Padding test
    suite.add("padding", |ctx| {
        let ui = div()
            .w(200.0)
            .h(200.0)
            .p_px(20.0)
            .bg(Color::rgba(0.4, 0.2, 0.2, 1.0))
            .child(div().w_full().h_full().rounded(8.0).bg(Color::rgba(0.2, 0.4, 0.6, 1.0)));

        let mut tree = RenderTree::from_element(&ui);
        tree.compute_layout(800.0, 600.0);
        tree.render(ctx.ctx());
    });

    // Rounded corners with layout
    suite.add("rounded_layout", |ctx| {
        let ui = div()
            .w(300.0)
            .h(200.0)
            .p_px(15.0)
            .rounded(20.0)
            .bg(Color::rgba(0.15, 0.15, 0.2, 1.0))
            .flex_col()
            .gap_px(10.0)
            .child(
                div()
                    .w_full()
                    .h(50.0)
                    .rounded(10.0)
                    .bg(Color::rgba(0.9, 0.3, 0.3, 1.0)),
            )
            .child(
                div()
                    .w_full()
                    .flex_grow()
                    .rounded(10.0)
                    .bg(Color::rgba(0.3, 0.6, 0.9, 1.0)),
            );

        let mut tree = RenderTree::from_element(&ui);
        tree.compute_layout(800.0, 600.0);
        tree.render(ctx.ctx());
    });

    // Card-like component
    suite.add("card_component", |ctx| {
        let card = div()
            .w(280.0)
            .h(180.0)
            .p_px(16.0)
            .rounded(16.0)
            .bg(Color::rgba(0.95, 0.95, 0.97, 1.0))
            .flex_col()
            .gap_px(12.0)
            // Header row
            .child(
                div()
                    .w_full()
                    .h(40.0)
                    .flex_row()
                    .gap_px(12.0)
                    .items_center()
                    // Avatar placeholder
                    .child(div().square(40.0).rounded(20.0).bg(Color::rgba(0.3, 0.5, 0.9, 1.0)))
                    // Title area
                    .child(
                        div()
                            .flex_grow()
                            .h(40.0)
                            .flex_col()
                            .gap_px(4.0)
                            .child(div().w(120.0).h(14.0).rounded(3.0).bg(Color::rgba(0.2, 0.2, 0.25, 1.0)))
                            .child(div().w(80.0).h(10.0).rounded(2.0).bg(Color::rgba(0.6, 0.6, 0.65, 1.0))),
                    ),
            )
            // Content area
            .child(
                div()
                    .w_full()
                    .flex_grow()
                    .rounded(8.0)
                    .bg(Color::rgba(0.9, 0.9, 0.92, 1.0)),
            )
            // Button row
            .child(
                div()
                    .w_full()
                    .h(36.0)
                    .flex_row()
                    .justify_end()
                    .gap_px(8.0)
                    .child(
                        div()
                            .w(80.0)
                            .h(36.0)
                            .rounded(8.0)
                            .bg(Color::rgba(0.85, 0.85, 0.88, 1.0)),
                    )
                    .child(
                        div()
                            .w(80.0)
                            .h(36.0)
                            .rounded(8.0)
                            .bg(Color::rgba(0.3, 0.5, 0.9, 1.0)),
                    ),
            );

        let mut tree = RenderTree::from_element(&card);
        tree.compute_layout(800.0, 600.0);

        // Center the card
        ctx.ctx()
            .push_transform(blinc_core::Transform::translate(60.0, 60.0));
        tree.render(ctx.ctx());
        ctx.ctx().pop_transform();
    });

    suite
}
