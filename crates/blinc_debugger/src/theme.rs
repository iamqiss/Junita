//! Custom dark theme constants for blinc_debugger
//!
//! Design system derived from the debugger dashboard mockup:
//! - Near-black background (#0A0A0A)
//! - Elevated card surfaces (#1C1C1E)
//! - Lime/green primary accent (#A3E635)
//! - Orange secondary accent (#F97316)

use blinc_core::Color;

/// Debugger color palette - dark mode optimized
pub struct DebuggerColors;

impl DebuggerColors {
    // Background colors
    pub const BG_BASE: Color = Color::from_rgb_u8(10, 10, 10); // #0A0A0A
    pub const BG_ELEVATED: Color = Color::from_rgb_u8(28, 28, 30); // #1C1C1E
    pub const BG_SURFACE: Color = Color::from_rgb_u8(38, 38, 40); // #262628
    pub const BG_HOVER: Color = Color::from_rgb_u8(48, 48, 52); // #303034

    // Border colors
    pub const BORDER_DEFAULT: Color = Color::from_rgb_u8(42, 42, 44); // #2A2A2C
    pub const BORDER_SUBTLE: Color = Color::from_rgb_u8(32, 32, 34); // #202022

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::from_rgb_u8(255, 255, 255); // #FFFFFF
    pub const TEXT_SECONDARY: Color = Color::from_rgb_u8(156, 163, 175); // #9CA3AF
    pub const TEXT_MUTED: Color = Color::from_rgb_u8(107, 114, 128); // #6B7280
    pub const TEXT_DISABLED: Color = Color::from_rgb_u8(75, 85, 99); // #4B5563

    // Primary accent - Lime green
    pub const PRIMARY: Color = Color::from_rgb_u8(163, 230, 53); // #A3E635
    pub const PRIMARY_HOVER: Color = Color::from_rgb_u8(132, 204, 22); // #84CC16

    // Secondary accent - Orange
    pub const SECONDARY: Color = Color::from_rgb_u8(249, 115, 22); // #F97316
    pub const SECONDARY_HOVER: Color = Color::from_rgb_u8(234, 88, 12); // #EA580C

    // Semantic colors
    pub const SUCCESS: Color = Color::from_rgb_u8(34, 197, 94); // #22C55E
    pub const WARNING: Color = Color::from_rgb_u8(250, 204, 21); // #FACC15
    pub const ERROR: Color = Color::from_rgb_u8(239, 68, 68); // #EF4444
    pub const INFO: Color = Color::from_rgb_u8(59, 130, 246); // #3B82F6

    // Diff colors (for tree diff visualization)
    pub const DIFF_ADDED: Color = Color::from_rgb_u8(34, 197, 94); // Green
    pub const DIFF_REMOVED: Color = Color::from_rgb_u8(239, 68, 68); // Red
    pub const DIFF_MODIFIED: Color = Color::from_rgb_u8(250, 204, 21); // Yellow
    pub const DIFF_UNCHANGED: Color = Color::from_rgb_u8(107, 114, 128); // Gray

    // Event type colors (for timeline)
    pub const EVENT_MOUSE: Color = Color::from_rgb_u8(163, 230, 53); // Lime
    pub const EVENT_KEYBOARD: Color = Color::from_rgb_u8(59, 130, 246); // Blue
    pub const EVENT_SCROLL: Color = Color::from_rgb_u8(249, 115, 22); // Orange
    pub const EVENT_FOCUS: Color = Color::from_rgb_u8(168, 85, 247); // Purple
    pub const EVENT_HOVER: Color = Color::from_rgb_u8(236, 72, 153); // Pink
}

/// Design tokens for the debugger UI
pub struct DebuggerTokens;

impl DebuggerTokens {
    // Border radius
    pub const RADIUS_SM: f32 = 6.0;
    pub const RADIUS_MD: f32 = 8.0;
    pub const RADIUS_LG: f32 = 12.0;
    pub const RADIUS_XL: f32 = 16.0;
    pub const RADIUS_FULL: f32 = 9999.0;

    // Spacing (based on 4px grid)
    pub const SPACE_1: f32 = 4.0;
    pub const SPACE_2: f32 = 8.0;
    pub const SPACE_3: f32 = 12.0;
    pub const SPACE_4: f32 = 16.0;
    pub const SPACE_5: f32 = 20.0;
    pub const SPACE_6: f32 = 24.0;
    pub const SPACE_8: f32 = 32.0;

    // Typography
    pub const FONT_SIZE_XS: f32 = 11.0;
    pub const FONT_SIZE_SM: f32 = 13.0;
    pub const FONT_SIZE_BASE: f32 = 14.0;
    pub const FONT_SIZE_LG: f32 = 16.0;
    pub const FONT_SIZE_XL: f32 = 18.0;
    pub const FONT_SIZE_2XL: f32 = 24.0;
    pub const FONT_SIZE_3XL: f32 = 30.0;
    pub const FONT_SIZE_4XL: f32 = 36.0;

    pub const FONT_WEIGHT_NORMAL: u16 = 400;
    pub const FONT_WEIGHT_MEDIUM: u16 = 500;
    pub const FONT_WEIGHT_SEMIBOLD: u16 = 600;
    pub const FONT_WEIGHT_BOLD: u16 = 700;

    // Panel dimensions (matching plan)
    pub const TREE_PANEL_WIDTH: f32 = 280.0;
    pub const INSPECTOR_WIDTH: f32 = 300.0;
    pub const TIMELINE_HEIGHT: f32 = 150.0;
    pub const HEADER_HEIGHT: f32 = 48.0;
    pub const PANEL_GAP: f32 = 8.0;
    pub const CARD_PADDING: f32 = 16.0;
    pub const CARD_GAP: f32 = 12.0;
}
