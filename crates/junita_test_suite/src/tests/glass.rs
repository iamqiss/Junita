//! Glass/Vibrancy effect tests
//!
//! Tests for Apple-style frosted glass and backdrop blur effects.
//! These tests require multi-pass rendering to capture backdrop content.

use crate::runner::TestSuite;
use junita_core::{Color, DrawContext, Rect};
use junita_gpu::GpuGlassPrimitive;
use junita_svg::SvgDocument;

/// Create the glass test suite
pub fn suite() -> TestSuite {
    let mut suite = TestSuite::new("glass");

    // Basic glass rectangle over solid background
    suite.add_glass("glass_basic", |ctx| {
        let c = ctx.ctx();

        // Colorful background to show blur effect
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.2, 0.4, 0.8, 1.0).into(),
        );

        // Some shapes behind the glass
        c.fill_rect(
            Rect::new(50.0, 50.0, 100.0, 100.0),
            8.0.into(),
            Color::RED.into(),
        );
        c.fill_rect(
            Rect::new(120.0, 80.0, 100.0, 100.0),
            8.0.into(),
            Color::GREEN.into(),
        );
        c.fill_rect(
            Rect::new(80.0, 120.0, 100.0, 100.0),
            8.0.into(),
            Color::YELLOW.into(),
        );

        // Glass overlay
        let glass = GpuGlassPrimitive::new(100.0, 100.0, 200.0, 120.0)
            .with_corner_radius(16.0)
            .with_tint(1.0, 1.0, 1.0, 0.2)
            .with_blur(20.0);
        ctx.add_glass(glass);
    });

    // Glass with different blur radii
    suite.add_glass("glass_blur_levels", |ctx| {
        let c = ctx.ctx();

        // Gradient-like background with shapes
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.1, 0.2, 0.4, 1.0).into(),
        );

        // Pattern of colored squares
        for i in 0..8 {
            for j in 0..6 {
                let color = if (i + j) % 2 == 0 {
                    Color::rgba(0.9, 0.3, 0.3, 1.0)
                } else {
                    Color::rgba(0.3, 0.3, 0.9, 1.0)
                };
                c.fill_rect(
                    Rect::new(i as f32 * 50.0, j as f32 * 50.0, 48.0, 48.0),
                    4.0.into(),
                    color.into(),
                );
            }
        }

        // Three glass panels with different blur amounts
        let glass_small = GpuGlassPrimitive::new(20.0, 100.0, 100.0, 100.0)
            .with_corner_radius(8.0)
            .with_blur(5.0)
            .with_tint(1.0, 1.0, 1.0, 0.15);
        ctx.add_glass(glass_small);

        let glass_medium = GpuGlassPrimitive::new(140.0, 100.0, 100.0, 100.0)
            .with_corner_radius(8.0)
            .with_blur(15.0)
            .with_tint(1.0, 1.0, 1.0, 0.15);
        ctx.add_glass(glass_medium);

        let glass_large = GpuGlassPrimitive::new(260.0, 100.0, 100.0, 100.0)
            .with_corner_radius(8.0)
            .with_blur(30.0)
            .with_tint(1.0, 1.0, 1.0, 0.15);
        ctx.add_glass(glass_large);
    });

    // Glass type presets (UltraThin, Thin, Regular, Thick, Chrome)
    suite.add_glass("glass_types", |ctx| {
        let c = ctx.ctx();

        // Colorful background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.8, 0.4, 0.2, 1.0).into(),
        );

        // Circles pattern
        for i in 0..10 {
            for j in 0..8 {
                c.fill_circle(
                    junita_core::Point::new(20.0 + i as f32 * 40.0, 20.0 + j as f32 * 40.0),
                    15.0,
                    Color::rgba(0.2, 0.6, 0.9, 0.8).into(),
                );
            }
        }

        // Five glass types side by side
        let glass_ultra_thin = GpuGlassPrimitive::new(10.0, 100.0, 70.0, 100.0)
            .ultra_thin()
            .with_corner_radius(8.0);
        ctx.add_glass(glass_ultra_thin);

        let glass_thin = GpuGlassPrimitive::new(90.0, 100.0, 70.0, 100.0)
            .thin()
            .with_corner_radius(8.0);
        ctx.add_glass(glass_thin);

        let glass_regular = GpuGlassPrimitive::new(170.0, 100.0, 70.0, 100.0)
            .regular()
            .with_corner_radius(8.0);
        ctx.add_glass(glass_regular);

        let glass_thick = GpuGlassPrimitive::new(250.0, 100.0, 70.0, 100.0)
            .thick()
            .with_corner_radius(8.0);
        ctx.add_glass(glass_thick);

        let glass_chrome = GpuGlassPrimitive::new(330.0, 100.0, 60.0, 100.0)
            .chrome()
            .with_corner_radius(8.0);
        ctx.add_glass(glass_chrome);
    });

    // Glass with colored tints
    suite.add_glass("glass_tinted", |ctx| {
        let c = ctx.ctx();

        // Neutral gray background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.5, 0.5, 0.5, 1.0).into(),
        );

        // Text-like stripes
        for i in 0..15 {
            c.fill_rect(
                Rect::new(20.0, 20.0 + i as f32 * 18.0, 360.0, 10.0),
                2.0.into(),
                Color::rgba(0.2, 0.2, 0.2, 1.0).into(),
            );
        }

        // Red tinted glass
        let glass_red = GpuGlassPrimitive::new(30.0, 80.0, 100.0, 140.0)
            .with_corner_radius(12.0)
            .with_blur(20.0)
            .with_tint(1.0, 0.3, 0.3, 0.3);
        ctx.add_glass(glass_red);

        // Green tinted glass
        let glass_green = GpuGlassPrimitive::new(150.0, 80.0, 100.0, 140.0)
            .with_corner_radius(12.0)
            .with_blur(20.0)
            .with_tint(0.3, 1.0, 0.3, 0.3);
        ctx.add_glass(glass_green);

        // Blue tinted glass
        let glass_blue = GpuGlassPrimitive::new(270.0, 80.0, 100.0, 140.0)
            .with_corner_radius(12.0)
            .with_blur(20.0)
            .with_tint(0.3, 0.3, 1.0, 0.3);
        ctx.add_glass(glass_blue);
    });

    // Glass with saturation adjustment
    suite.add_glass("glass_saturation", |ctx| {
        let c = ctx.ctx();

        // Very colorful background
        c.fill_rect(
            Rect::new(0.0, 0.0, 200.0, 300.0),
            0.0.into(),
            Color::rgba(1.0, 0.0, 0.5, 1.0).into(),
        );
        c.fill_rect(
            Rect::new(200.0, 0.0, 200.0, 300.0),
            0.0.into(),
            Color::rgba(0.0, 0.8, 1.0, 1.0).into(),
        );

        // Colored circles
        c.fill_circle(
            junita_core::Point::new(100.0, 150.0),
            60.0,
            Color::YELLOW.into(),
        );
        c.fill_circle(
            junita_core::Point::new(300.0, 150.0),
            60.0,
            Color::YELLOW.into(),
        );

        // Glass with reduced saturation (more grayscale blur)
        let glass_desat = GpuGlassPrimitive::new(50.0, 100.0, 100.0, 100.0)
            .with_corner_radius(12.0)
            .with_blur(20.0)
            .with_saturation(0.3) // Low saturation
            .with_tint(1.0, 1.0, 1.0, 0.1);
        ctx.add_glass(glass_desat);

        // Glass with enhanced saturation
        let glass_sat = GpuGlassPrimitive::new(250.0, 100.0, 100.0, 100.0)
            .with_corner_radius(12.0)
            .with_blur(20.0)
            .with_saturation(1.5) // High saturation
            .with_tint(1.0, 1.0, 1.0, 0.1);
        ctx.add_glass(glass_sat);
    });

    // Glass with brightness adjustment
    suite.add_glass("glass_brightness", |ctx| {
        let c = ctx.ctx();

        // Medium gray background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.4, 0.4, 0.4, 1.0).into(),
        );

        // Pattern
        for i in 0..20 {
            c.fill_circle(
                junita_core::Point::new(20.0 + i as f32 * 20.0, 150.0),
                8.0,
                Color::WHITE.into(),
            );
        }

        // Darker glass (low brightness)
        let glass_dark = GpuGlassPrimitive::new(50.0, 80.0, 130.0, 140.0)
            .with_corner_radius(12.0)
            .with_blur(15.0)
            .with_brightness(0.6)
            .with_tint(0.0, 0.0, 0.0, 0.2);
        ctx.add_glass(glass_dark);

        // Brighter glass (high brightness)
        let glass_bright = GpuGlassPrimitive::new(220.0, 80.0, 130.0, 140.0)
            .with_corner_radius(12.0)
            .with_blur(15.0)
            .with_brightness(1.4)
            .with_tint(1.0, 1.0, 1.0, 0.2);
        ctx.add_glass(glass_bright);
    });

    // Glass with corner radius variations
    suite.add_glass("glass_corners", |ctx| {
        let c = ctx.ctx();

        // Gradient background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.2, 0.3, 0.5, 1.0).into(),
        );

        // Grid of small shapes
        for i in 0..16 {
            for j in 0..12 {
                c.fill_rect(
                    Rect::new(i as f32 * 25.0, j as f32 * 25.0, 20.0, 20.0),
                    2.0.into(),
                    Color::rgba(0.9, 0.7, 0.3, 0.8).into(),
                );
            }
        }

        // Sharp corners
        let glass_sharp = GpuGlassPrimitive::new(30.0, 100.0, 100.0, 100.0)
            .with_corner_radius(0.0)
            .with_blur(20.0)
            .with_tint(1.0, 1.0, 1.0, 0.2);
        ctx.add_glass(glass_sharp);

        // Medium corners
        let glass_medium = GpuGlassPrimitive::new(150.0, 100.0, 100.0, 100.0)
            .with_corner_radius(16.0)
            .with_blur(20.0)
            .with_tint(1.0, 1.0, 1.0, 0.2);
        ctx.add_glass(glass_medium);

        // Very rounded (pill-like)
        let glass_rounded = GpuGlassPrimitive::new(270.0, 100.0, 100.0, 100.0)
            .with_corner_radius(50.0) // Full pill shape
            .with_blur(20.0)
            .with_tint(1.0, 1.0, 1.0, 0.2);
        ctx.add_glass(glass_rounded);
    });

    // Glass modal dialog pattern
    suite.add_glass("glass_modal_dialog", |ctx| {
        let c = ctx.ctx();

        // App-like background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.15, 0.15, 0.2, 1.0).into(),
        );

        // Fake app content (cards)
        c.fill_rect(
            Rect::new(20.0, 20.0, 160.0, 80.0),
            8.0.into(),
            Color::rgba(0.25, 0.25, 0.35, 1.0).into(),
        );
        c.fill_rect(
            Rect::new(20.0, 110.0, 160.0, 80.0),
            8.0.into(),
            Color::rgba(0.25, 0.25, 0.35, 1.0).into(),
        );
        c.fill_rect(
            Rect::new(220.0, 20.0, 160.0, 170.0),
            8.0.into(),
            Color::rgba(0.3, 0.25, 0.4, 1.0).into(),
        );

        // Accent elements
        c.fill_circle(
            junita_core::Point::new(300.0, 100.0),
            40.0,
            Color::rgba(0.4, 0.6, 1.0, 0.8).into(),
        );

        // Dark overlay/scrim
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.0, 0.0, 0.0, 0.4).into(),
        );

        // Glass modal
        let modal = GpuGlassPrimitive::new(80.0, 60.0, 240.0, 180.0)
            .with_corner_radius(20.0)
            .with_blur(25.0)
            .with_tint(0.2, 0.2, 0.25, 0.7)
            .with_saturation(0.8);
        ctx.add_glass(modal);
    });

    // Glass sidebar pattern
    suite.add_glass("glass_sidebar", |ctx| {
        let c = ctx.ctx();

        // Image-like colorful background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.3, 0.5, 0.7, 1.0).into(),
        );

        // "Photo" content
        c.fill_rect(
            Rect::new(50.0, 30.0, 300.0, 200.0),
            12.0.into(),
            Color::rgba(0.8, 0.6, 0.4, 1.0).into(),
        );
        c.fill_circle(
            junita_core::Point::new(200.0, 130.0),
            50.0,
            Color::rgba(1.0, 0.8, 0.3, 1.0).into(),
        );

        // Glass sidebar
        let sidebar = GpuGlassPrimitive::new(0.0, 0.0, 80.0, 300.0)
            .with_corner_radius(0.0)
            .thick()
            .with_tint(0.1, 0.1, 0.15, 0.6);
        ctx.add_glass(sidebar);
    });

    // Overlapping glass panels
    suite.add_glass("glass_overlapping", |ctx| {
        let c = ctx.ctx();

        // Colorful background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.8, 0.2, 0.4, 1.0).into(),
        );

        // Background shapes
        c.fill_rect(
            Rect::new(40.0, 40.0, 120.0, 120.0),
            16.0.into(),
            Color::YELLOW.into(),
        );
        c.fill_rect(
            Rect::new(240.0, 140.0, 120.0, 120.0),
            16.0.into(),
            Color::CYAN.into(),
        );

        // First glass panel (back)
        let glass1 = GpuGlassPrimitive::new(60.0, 60.0, 150.0, 150.0)
            .with_corner_radius(16.0)
            .with_blur(20.0)
            .with_tint(0.0, 0.0, 1.0, 0.2);
        ctx.add_glass(glass1);

        // Second glass panel (overlapping)
        let glass2 = GpuGlassPrimitive::new(140.0, 100.0, 150.0, 150.0)
            .with_corner_radius(16.0)
            .with_blur(20.0)
            .with_tint(0.0, 1.0, 0.0, 0.2);
        ctx.add_glass(glass2);
    });

    // iOS-style notification card
    suite.add_glass("glass_notification", |ctx| {
        let c = ctx.ctx();

        // Lock screen background (gradient-like)
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 150.0),
            0.0.into(),
            Color::rgba(0.1, 0.3, 0.5, 1.0).into(),
        );
        c.fill_rect(
            Rect::new(0.0, 150.0, 400.0, 150.0),
            0.0.into(),
            Color::rgba(0.2, 0.1, 0.4, 1.0).into(),
        );

        // Notification card
        let notification = GpuGlassPrimitive::new(20.0, 80.0, 360.0, 80.0)
            .with_corner_radius(16.0)
            .regular()
            .with_tint(0.95, 0.95, 0.97, 0.7)
            .with_saturation(1.2);
        ctx.add_glass(notification);
    });

    // macOS-style menu bar
    suite.add_glass("glass_menubar", |ctx| {
        let c = ctx.ctx();

        // Desktop wallpaper (colorful)
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.2, 0.5, 0.8, 1.0).into(),
        );

        // Window content
        c.fill_rect(
            Rect::new(50.0, 60.0, 300.0, 200.0),
            8.0.into(),
            Color::WHITE.into(),
        );

        // Menu bar at top
        let menubar = GpuGlassPrimitive::new(0.0, 0.0, 400.0, 28.0)
            .with_corner_radius(0.0)
            .thin()
            .with_tint(0.95, 0.95, 0.97, 0.8)
            .with_saturation(1.0)
            .with_brightness(1.1);
        ctx.add_glass(menubar);

        // Dock at bottom
        let dock = GpuGlassPrimitive::new(80.0, 260.0, 240.0, 35.0)
            .with_corner_radius(10.0)
            .thick()
            .with_tint(0.5, 0.5, 0.5, 0.4);
        ctx.add_glass(dock);
    });

    // Glass with drop shadows
    suite.add_glass("glass_shadows", |ctx| {
        let c = ctx.ctx();

        // Light background to show shadows clearly
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 300.0),
            0.0.into(),
            Color::rgba(0.95, 0.95, 0.97, 1.0).into(),
        );

        // Subtle pattern
        for i in 0..20 {
            for j in 0..15 {
                c.fill_rect(
                    Rect::new(i as f32 * 20.0, j as f32 * 20.0, 18.0, 18.0),
                    2.0.into(),
                    Color::rgba(0.85, 0.85, 0.87, 1.0).into(),
                );
            }
        }

        // Glass with subtle shadow
        let glass_subtle = GpuGlassPrimitive::new(30.0, 80.0, 100.0, 120.0)
            .with_corner_radius(16.0)
            .with_blur(15.0)
            .with_tint(1.0, 1.0, 1.0, 0.3)
            .with_shadow(10.0, 0.15);
        ctx.add_glass(glass_subtle);

        // Glass with medium shadow
        let glass_medium = GpuGlassPrimitive::new(150.0, 80.0, 100.0, 120.0)
            .with_corner_radius(16.0)
            .with_blur(15.0)
            .with_tint(1.0, 1.0, 1.0, 0.3)
            .with_shadow(20.0, 0.3);
        ctx.add_glass(glass_medium);

        // Glass with strong shadow
        let glass_strong = GpuGlassPrimitive::new(270.0, 80.0, 100.0, 120.0)
            .with_corner_radius(16.0)
            .with_blur(15.0)
            .with_tint(1.0, 1.0, 1.0, 0.3)
            .with_shadow(30.0, 0.5);
        ctx.add_glass(glass_strong);
    });

    // Glass with offset shadows (floating card effect)
    suite.add_glass("glass_shadow_offset", |ctx| {
        let c = ctx.ctx();

        // Gradient-like background
        c.fill_rect(
            Rect::new(0.0, 0.0, 400.0, 150.0),
            0.0.into(),
            Color::rgba(0.4, 0.5, 0.7, 1.0).into(),
        );
        c.fill_rect(
            Rect::new(0.0, 150.0, 400.0, 150.0),
            0.0.into(),
            Color::rgba(0.5, 0.4, 0.6, 1.0).into(),
        );

        // Some colorful shapes
        c.fill_circle(
            junita_core::Point::new(80.0, 150.0),
            40.0,
            Color::rgba(1.0, 0.5, 0.3, 0.8).into(),
        );
        c.fill_circle(
            junita_core::Point::new(320.0, 150.0),
            40.0,
            Color::rgba(0.3, 0.8, 0.5, 0.8).into(),
        );

        // Floating card with bottom-right shadow
        let card1 = GpuGlassPrimitive::new(60.0, 60.0, 120.0, 80.0)
            .with_corner_radius(12.0)
            .with_blur(20.0)
            .with_tint(1.0, 1.0, 1.0, 0.4)
            .with_shadow_offset(15.0, 0.4, 8.0, 8.0);
        ctx.add_glass(card1);

        // Floating card with bottom shadow (iOS style)
        let card2 = GpuGlassPrimitive::new(220.0, 60.0, 120.0, 80.0)
            .with_corner_radius(12.0)
            .with_blur(20.0)
            .with_tint(1.0, 1.0, 1.0, 0.4)
            .with_shadow_offset(20.0, 0.35, 0.0, 12.0);
        ctx.add_glass(card2);

        // Bottom notification with spread shadow
        let notification = GpuGlassPrimitive::new(50.0, 200.0, 300.0, 60.0)
            .with_corner_radius(20.0)
            .with_blur(25.0)
            .with_tint(0.1, 0.1, 0.15, 0.7)
            .with_shadow_offset(25.0, 0.5, 0.0, 15.0);
        ctx.add_glass(notification);
    });

    // iOS 26 Liquid Glass Music Player (based on reference image)
    // This test recreates the Apple Control Center music player widget
    suite.add_glass("music_player", |ctx| {
        use junita_core::Point;

        // Scale factor for high-resolution rendering (2x for 800x600)
        let scale = 2.0;

        // Layout constants - iPhone-like aspect ratio (scaled)
        let width = 400.0 * scale;
        let height = 300.0 * scale;

        // Player card dimensions (scaled)
        let player_x = 30.0 * scale;
        let player_y = 30.0 * scale;
        let player_w = 340.0 * scale;
        let player_h = 140.0 * scale;
        let player_radius = 28.0 * scale;

        // Progress bar (slider) - iOS style with no knob
        // Leave space for time labels on both sides
        let time_label_width = 35.0 * scale;
        let bar_x = player_x + 20.0 * scale + time_label_width;
        let bar_y = player_y + 50.0 * scale;
        let bar_w = player_w - 40.0 * scale - (time_label_width * 2.0);
        let bar_h = 7.0 * scale; // Track thickness
        let progress = 0.08; // ~0:10 of 3:34

        // Control buttons layout
        let controls_y = player_y + 85.0 * scale;
        let controls_center_x = player_x + player_w / 2.0;
        let btn_spacing = 70.0 * scale;


        // First, draw all background primitives (will be blurred behind glass)
        {
            let c = ctx.ctx();

            // Vibrant multicolor background pattern
            // Base gradient: purple to orange
            c.fill_rect(
                Rect::new(0.0, 0.0, width, height),
                0.0.into(),
                Color::rgba(0.4, 0.2, 0.6, 1.0).into(),
            );

            // Large colorful shapes for interesting blur (scaled)
            // Pink/magenta blob top-left
            c.fill_circle(
                Point::new(80.0 * scale, 60.0 * scale),
                100.0 * scale,
                Color::rgba(0.95, 0.3, 0.5, 1.0).into(),
            );

            // Cyan/teal blob center-right
            c.fill_circle(
                Point::new(320.0 * scale, 120.0 * scale),
                90.0 * scale,
                Color::rgba(0.2, 0.8, 0.85, 1.0).into(),
            );

            // Orange blob bottom
            c.fill_circle(
                Point::new(180.0 * scale, 260.0 * scale),
                80.0 * scale,
                Color::rgba(1.0, 0.5, 0.2, 1.0).into(),
            );

            // Yellow accent
            c.fill_circle(
                Point::new(350.0 * scale, 240.0 * scale),
                60.0 * scale,
                Color::rgba(1.0, 0.85, 0.2, 1.0).into(),
            );

            // Green accent bottom-left
            c.fill_circle(
                Point::new(50.0 * scale, 220.0 * scale),
                70.0 * scale,
                Color::rgba(0.3, 0.9, 0.4, 1.0).into(),
            );

            // Blue accent top-right
            c.fill_rect(
                Rect::new(280.0 * scale, 0.0, 120.0 * scale, 80.0 * scale),
                (20.0 * scale).into(),
                Color::rgba(0.3, 0.4, 0.95, 1.0).into(),
            );
        }

        // Add all glass primitives
        // Main player card - iOS liquid glass style with shadow
        let player_glass = GpuGlassPrimitive::new(player_x, player_y, player_w, player_h)
            .with_corner_radius(player_radius)
            .with_blur(30.0 * scale)
            .with_tint(0.12, 0.12, 0.14, 0.55)
            .with_saturation(0.85)
            .with_brightness(1.05)
            .with_border_thickness(0.6 * scale)
            .with_light_angle_degrees(-45.0)
            .with_shadow_offset(20.0 * scale, 0.35, 0.0, 10.0 * scale);
        ctx.add_glass(player_glass);

        // Slider track as glass element (for refraction effect)
        let slider_track_glass = GpuGlassPrimitive::new(bar_x, bar_y, bar_w, bar_h)
            .with_corner_radius(bar_h / 2.0)
            .with_blur(25.0 * scale)  // Higher blur for more frosted effect
            .with_tint(1.0, 1.0, 1.0, 0.65)  // More opaque white tint
            .with_saturation(0.3)  // Very low saturation for whiter appearance
            .with_brightness(1.3)  // Brighter
            .with_border_thickness(0.0);
        ctx.add_glass(slider_track_glass);


        // Pill button settings (shared between left and right pills)
        let pill_icon_size = 20.0 * scale;
        let pill_padding = 12.0 * scale;
        let pill_w = pill_icon_size + pill_padding * 2.0;
        let pill_h = pill_icon_size + pill_padding * 2.0;
        // Center pills vertically with the control icons (32.0 * scale height)
        let control_icon_height = 32.0 * scale;
        let pill_y = controls_y + (control_icon_height - pill_h) / 2.0;

        // AirPlay button glass pill (left side)
        let airplay_pill_x = player_x + 16.0 * scale;

        // Glass pill with etched/inset appearance - flat blur with visible border
        let airplay_pill_glass = GpuGlassPrimitive::new(airplay_pill_x, pill_y, pill_w, pill_h)
            .with_corner_radius(pill_h / 2.0)
            .with_blur(20.0 * scale)
            .with_tint(0.92, 0.92, 0.94, 0.4)
            .with_saturation(0.8)
            .with_brightness(0.95)
            .with_border_thickness(1.0 * scale)
            .flat();
        ctx.add_glass(airplay_pill_glass);

        // Radio/wave button glass pill (right side)
        let radio_pill_x = player_x + player_w - 16.0 * scale - pill_w;

        let radio_pill_glass = GpuGlassPrimitive::new(radio_pill_x, pill_y, pill_w, pill_h)
            .with_corner_radius(pill_h / 2.0)
            .with_blur(20.0 * scale)
            .with_tint(0.92, 0.92, 0.94, 0.4)
            .with_saturation(0.8)
            .with_brightness(0.95)
            .with_border_thickness(1.0 * scale)
            .flat();
        ctx.add_glass(radio_pill_glass);

        // Draw foreground elements ON TOP of glass (not blurred)
        {
            let fg = ctx.foreground();

            // Progress fill (white, full opacity) - on top of glass slider track
            fg.fill_rect(
                Rect::new(bar_x, bar_y, bar_w * progress, bar_h),
                (bar_h / 2.0).into(),
                Color::rgba(1.0, 1.0, 1.0, 1.0).into(),
            );

            // Rewind button (SVG icon - mirrored forward)
            let rewind_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                <path d="M236.3 107.1C247.9 96 265 92.9 279.7 99.2C294.4 105.5 304 120 304 136L304 272.3L476.3 107.2C487.9 96 505 92.9 519.7 99.2C534.4 105.5 544 120 544 136L544 504C544 520 534.4 534.5 519.7 540.8C505 547.1 487.9 544 476.3 532.9L304 367.7L304 504C304 520 294.4 534.5 279.7 540.8C265 547.1 247.9 544 236.3 532.9L44.3 348.9C36.4 341.4 32 330.9 32 320C32 309.1 36.5 298.7 44.3 291.1L236.3 107.1z" fill="white"/>
            </svg>"#;
            if let Ok(doc) = SvgDocument::from_str(rewind_svg) {
                let icon_size = 32.0 * scale;
                let rew_x = controls_center_x - btn_spacing - icon_size / 2.0;
                doc.render_fit(fg, Rect::new(rew_x, controls_y, icon_size, icon_size));
            }

            // Pause button (SVG icon)
            let pause_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                <path d="M176 96C149.5 96 128 117.5 128 144L128 496C128 522.5 149.5 544 176 544L240 544C266.5 544 288 522.5 288 496L288 144C288 117.5 266.5 96 240 96L176 96zM400 96C373.5 96 352 117.5 352 144L352 496C352 522.5 373.5 544 400 544L464 544C490.5 544 512 522.5 512 496L512 144C512 117.5 490.5 96 464 96L400 96z" fill="white"/>
            </svg>"#;
            if let Ok(doc) = SvgDocument::from_str(pause_svg) {
                let pause_size = 32.0 * scale;
                let pause_x = controls_center_x - pause_size / 2.0;
                doc.render_fit(fg, Rect::new(pause_x, controls_y, pause_size, pause_size));
            }

            // Fast-forward button (SVG icon)
            let forward_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                <path d="M403.7 107.1C392.1 96 375 92.9 360.3 99.2C345.6 105.5 336 120 336 136L336 272.3L163.7 107.2C152.1 96 135 92.9 120.3 99.2C105.6 105.5 96 120 96 136L96 504C96 520 105.6 534.5 120.3 540.8C135 547.1 152.1 544 163.7 532.9L336 367.7L336 504C336 520 345.6 534.5 360.3 540.8C375 547.1 392.1 544 403.7 532.9L595.7 348.9C603.6 341.4 608 330.9 608 320C608 309.1 603.5 298.7 595.7 291.1L403.7 107.1z" fill="white"/>
            </svg>"#;
            if let Ok(doc) = SvgDocument::from_str(forward_svg) {
                let icon_size = 32.0 * scale;
                let ff_x = controls_center_x + btn_spacing - icon_size / 2.0;
                doc.render_fit(fg, Rect::new(ff_x, controls_y, icon_size, icon_size));
            }

            // Audio wave icon (Lucide audio-lines) - top right of player
            let audio_icon_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M2 10v3"/><path d="M6 6v11"/><path d="M10 3v18"/><path d="M14 8v7"/><path d="M18 5v13"/><path d="M22 10v3"/></svg>"##;
            if let Ok(doc) = SvgDocument::from_str(audio_icon_svg) {
                let icon_size = 22.0 * scale;
                let audio_icon_x = player_x + player_w - 48.0 * scale;
                let audio_icon_y = player_y + 12.0 * scale;
                doc.render_fit(fg, Rect::new(audio_icon_x, audio_icon_y, icon_size, icon_size));
            }

            // AirPlay icon (Lucide airplay) - in glass pill on left side
            let airplay_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"/><path d="m12 15 5 6H7Z"/></svg>"##;
            if let Ok(doc) = SvgDocument::from_str(airplay_svg) {
                let airplay_icon_x = airplay_pill_x + pill_padding;
                let airplay_icon_y = pill_y + pill_padding;
                doc.render_fit(fg, Rect::new(airplay_icon_x, airplay_icon_y, pill_icon_size, pill_icon_size));
            }

            // Radio icon (Lucide radio) - in glass pill on right side
            // Use filled circle for smoother rendering (stroked paths have jagged edges without AA)
            let radio_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16.247 7.761a6 6 0 0 1 0 8.478"/><path d="M19.075 4.933a10 10 0 0 1 0 14.134"/><path d="M4.925 19.067a10 10 0 0 1 0-14.134"/><path d="M7.753 16.239a6 6 0 0 1 0-8.478"/><circle cx="12" cy="12" r="2.5" fill="white"/></svg>"##;
            if let Ok(doc) = SvgDocument::from_str(radio_svg) {
                let radio_icon_x = radio_pill_x + pill_padding;
                let radio_icon_y = pill_y + pill_padding;
                doc.render_fit(fg, Rect::new(radio_icon_x, radio_icon_y, pill_icon_size, pill_icon_size));
            }
        }

        // Title label above the slider (horizontally centered)
        let title_font_size = 14.0 * scale;
        let title_y = bar_y - 20.0 * scale;  // Position above the slider
        // Approximate text width: "Junita UI 0.1.0" is 14 chars, ~0.5 * font_size per char
        let title_text = "Junita UI 0.1.0";
        let approx_title_width = title_text.len() as f32 * title_font_size * 0.5;
        ctx.draw_text_centered(
            title_text,
            player_x + (player_w - approx_title_width) / 2.0,  // Center horizontally
            title_y,
            title_font_size,
            [1.0, 1.0, 1.0, 0.95],
        );

        // Time labels (rendered as text on top of everything)
        let font_size = 11.0 * scale;
        // Use centered text anchor so y coordinate is the vertical center
        let slider_center_y = bar_y + bar_h / 2.0;
        // Left time label (elapsed)
        ctx.draw_text_centered(
            "0:10",
            player_x + 20.0 * scale,
            slider_center_y,
            font_size,
            [1.0, 1.0, 1.0, 0.85],
        );
        // Right time label (remaining)
        ctx.draw_text_centered(
            "-3:24",
            player_x + player_w - 20.0 * scale - 28.0 * scale,
            slider_center_y,
            font_size,
            [1.0, 1.0, 1.0, 0.85],
        );
    });

    // SVG stroke test - white filled SVG on colored background
    suite.add_glass("svg_stroke_test", |ctx| {
        let c = ctx.ctx();
        let scale = 2.0; // Match new default resolution

        // Solid colored background
        c.fill_rect(
            Rect::new(0.0, 0.0, 800.0, 600.0),
            0.0.into(),
            Color::rgba(0.2, 0.4, 0.8, 1.0).into(),
        );

        // AirPlay icon (stroked SVG - should have visible stroke)
        let airplay_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"/><path d="m12 15 5 6H7Z"/></svg>"##;
        if let Ok(doc) = SvgDocument::from_str(airplay_svg) {
            doc.render_fit(c, Rect::new(50.0 * scale, 50.0 * scale, 80.0 * scale, 80.0 * scale));
        }

        // Audio lines icon (stroked SVG)
        let audio_icon_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M2 10v3"/><path d="M6 6v11"/><path d="M10 3v18"/><path d="M14 8v7"/><path d="M18 5v13"/><path d="M22 10v3"/></svg>"##;
        if let Ok(doc) = SvgDocument::from_str(audio_icon_svg) {
            doc.render_fit(c, Rect::new(160.0 * scale, 50.0 * scale, 80.0 * scale, 80.0 * scale));
        }

        // Forward button (filled SVG - should NOT have stroke outline)
        let forward_svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
            <path d="M403.7 107.1C392.1 96 375 92.9 360.3 99.2C345.6 105.5 336 120 336 136L336 272.3L163.7 107.2C152.1 96 135 92.9 120.3 99.2C105.6 105.5 96 120 96 136L96 504C96 520 105.6 534.5 120.3 540.8C135 547.1 152.1 544 163.7 532.9L336 367.7L336 504C336 520 345.6 534.5 360.3 540.8C375 547.1 392.1 544 403.7 532.9L595.7 348.9C603.6 341.4 608 330.9 608 320C608 309.1 603.5 298.7 595.7 291.1L403.7 107.1z" fill="white"/>
        </svg>"#;
        if let Ok(doc) = SvgDocument::from_str(forward_svg) {
            doc.render_fit(c, Rect::new(270.0 * scale, 50.0 * scale, 80.0 * scale, 80.0 * scale));
        }

        // Labels
        ctx.draw_text("Stroked SVGs", 100.0 * scale, 150.0 * scale, 14.0 * scale, [1.0, 1.0, 1.0, 1.0]);
        ctx.draw_text("Filled SVG", 290.0 * scale, 150.0 * scale, 14.0 * scale, [1.0, 1.0, 1.0, 1.0]);
    });

    suite
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness::TestHarness;

    #[test]
    #[ignore] // Requires GPU
    fn run_glass_suite() {
        let harness = TestHarness::new().unwrap();
        let mut suite = suite();

        for case in suite.cases.drain(..) {
            let result = harness.run_glass_test(&case.name, case.test_fn).unwrap();
            assert!(result.is_passed(), "Test {} failed", case.name);
        }
    }
}
