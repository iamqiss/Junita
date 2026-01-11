//! Timeline Panel - Event timeline with scrubber
//!
//! Displays:
//! - Timeline ruler with time markers
//! - Event markers (colored by type)
//! - Playback scrubber
//! - Playback controls (play, pause, step, speed)

use crate::theme::{DebuggerColors, DebuggerTokens};
use blinc_core::Color;
use blinc_layout::prelude::*;
use blinc_recorder::replay::ReplayState;
use blinc_recorder::{RecordedEvent, Timestamp, TimestampedEvent};

/// Timeline panel state
pub struct TimelinePanelState {
    /// Current playback position
    pub position: Timestamp,
    /// Total duration
    pub duration: Timestamp,
    /// Playback state
    pub playback_state: ReplayState,
    /// Playback speed multiplier
    pub speed: f64,
}

impl Default for TimelinePanelState {
    fn default() -> Self {
        Self {
            position: Timestamp::zero(),
            duration: Timestamp::zero(),
            playback_state: ReplayState::Idle,
            speed: 1.0,
        }
    }
}

/// Timeline panel component
pub struct TimelinePanel<'a> {
    events: &'a [TimestampedEvent],
    state: &'a TimelinePanelState,
}

impl<'a> TimelinePanel<'a> {
    pub fn new(events: &'a [TimestampedEvent], state: &'a TimelinePanelState) -> Self {
        Self { events, state }
    }

    /// Build the timeline panel
    pub fn build(self) -> impl ElementBuilder {
        div()
            .w_full()
            .h(DebuggerTokens::TIMELINE_HEIGHT)
            .bg(DebuggerColors::BG_ELEVATED)
            .border_t(1.0)
            .border_color(DebuggerColors::BORDER_SUBTLE)
            .flex_col()
            .child(self.controls())
            .child(self.timeline_track())
    }

    fn controls(&self) -> impl ElementBuilder {
        div()
            .h(40.0)
            .px(DebuggerTokens::SPACE_4)
            .flex_row()
            .items_center()
            .justify_between()
            .child(
                // Playback controls
                div()
                    .flex_row()
                    .gap(DebuggerTokens::SPACE_2)
                    .child(self.control_button("\u{23EE}", "Step back")) // ⏮
                    .child(self.play_pause_button())
                    .child(self.control_button("\u{23ED}", "Step forward")), // ⏭
            )
            .child(
                // Time display
                div()
                    .flex_row()
                    .items_center()
                    .gap(DebuggerTokens::SPACE_2)
                    .child(
                        text(self.format_time(self.state.position))
                            .size(DebuggerTokens::FONT_SIZE_SM)
                            .color(DebuggerColors::TEXT_PRIMARY),
                    )
                    .child(
                        text("/")
                            .size(DebuggerTokens::FONT_SIZE_SM)
                            .color(DebuggerColors::TEXT_MUTED),
                    )
                    .child(
                        text(self.format_time(self.state.duration))
                            .size(DebuggerTokens::FONT_SIZE_SM)
                            .color(DebuggerColors::TEXT_SECONDARY),
                    ),
            )
            .child(
                // Speed control
                div()
                    .flex_row()
                    .items_center()
                    .gap(DebuggerTokens::SPACE_2)
                    .child(
                        text("Speed:")
                            .size(DebuggerTokens::FONT_SIZE_XS)
                            .color(DebuggerColors::TEXT_MUTED),
                    )
                    .child(self.speed_button(0.5))
                    .child(self.speed_button(1.0))
                    .child(self.speed_button(2.0)),
            )
    }

    fn control_button(&self, icon: &str, _tooltip: &str) -> impl ElementBuilder {
        div()
            .w(32.0)
            .h(32.0)
            .rounded(DebuggerTokens::RADIUS_MD)
            .bg(DebuggerColors::BG_SURFACE)
            .items_center()
            .justify_center()
            .cursor_pointer()
            .child(
                text(icon)
                    .size(DebuggerTokens::FONT_SIZE_LG)
                    .color(DebuggerColors::TEXT_SECONDARY),
            )
    }

    fn play_pause_button(&self) -> impl ElementBuilder {
        let is_playing = self.state.playback_state == ReplayState::Playing;
        let icon = if is_playing { "\u{23F8}" } else { "\u{25B6}" }; // ⏸ or ▶

        div()
            .w(40.0)
            .h(32.0)
            .rounded(DebuggerTokens::RADIUS_MD)
            .bg(DebuggerColors::PRIMARY)
            .items_center()
            .justify_center()
            .cursor_pointer()
            .child(
                text(icon)
                    .size(DebuggerTokens::FONT_SIZE_LG)
                    .color(DebuggerColors::BG_BASE),
            )
    }

    fn speed_button(&self, speed: f64) -> impl ElementBuilder {
        let is_active = (self.state.speed - speed).abs() < 0.01;
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
            .px(DebuggerTokens::SPACE_2)
            .py(DebuggerTokens::SPACE_1)
            .rounded(DebuggerTokens::RADIUS_SM)
            .bg(bg)
            .cursor_pointer()
            .child(
                text(format!("{:.1}x", speed))
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(text_color),
            )
    }

    fn timeline_track(&self) -> impl ElementBuilder {
        let progress = if self.state.duration.as_micros() > 0 {
            self.state.position.as_micros() as f32 / self.state.duration.as_micros() as f32
        } else {
            0.0
        };

        div()
            .flex_grow()
            .px(DebuggerTokens::SPACE_4)
            .py(DebuggerTokens::SPACE_2)
            .flex_col()
            .gap(DebuggerTokens::SPACE_2)
            .child(
                // Event markers track
                self.event_markers_track(),
            )
            .child(
                // Scrubber track
                div()
                    .w_full()
                    .h(8.0)
                    .bg(DebuggerColors::BG_SURFACE)
                    .rounded(DebuggerTokens::RADIUS_FULL)
                    .relative()
                    .cursor_pointer()
                    .child(
                        // Progress fill
                        div()
                            .h_full()
                            .w_pct(progress * 100.0)
                            .bg(DebuggerColors::PRIMARY)
                            .rounded(DebuggerTokens::RADIUS_FULL),
                    )
                    .child(
                        // Scrubber handle
                        div()
                            .absolute()
                            .left_pct(progress * 100.0)
                            .top(-4.0)
                            .w(16.0)
                            .h(16.0)
                            .rounded_full()
                            .bg(DebuggerColors::PRIMARY)
                            .border(2.0)
                            .border_color(Color::WHITE)
                            .ml(-8.0), // Center the handle
                    ),
            )
            .child(
                // Time markers
                self.time_markers(),
            )
    }

    fn event_markers_track(&self) -> impl ElementBuilder {
        // TODO: Render actual event markers from self.events
        div()
            .w_full()
            .h(24.0)
            .relative()
            .children((0..10).map(|i| {
                let x_pct = (i as f32 + 1.0) * 9.0; // Spread across track
                let color = match i % 5 {
                    0 => DebuggerColors::EVENT_MOUSE,
                    1 => DebuggerColors::EVENT_KEYBOARD,
                    2 => DebuggerColors::EVENT_SCROLL,
                    3 => DebuggerColors::EVENT_FOCUS,
                    _ => DebuggerColors::EVENT_HOVER,
                };
                div()
                    .absolute()
                    .left_pct(x_pct)
                    .top(4.0)
                    .w(4.0)
                    .h(16.0)
                    .rounded(2.0)
                    .bg(color)
            }))
    }

    fn time_markers(&self) -> impl ElementBuilder {
        div()
            .w_full()
            .h(16.0)
            .flex_row()
            .justify_between()
            .child(
                text("0:00")
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(DebuggerColors::TEXT_MUTED),
            )
            .child(
                text(self.format_time(self.state.duration))
                    .size(DebuggerTokens::FONT_SIZE_XS)
                    .color(DebuggerColors::TEXT_MUTED),
            )
    }

    fn format_time(&self, ts: Timestamp) -> String {
        let total_secs = ts.as_micros() / 1_000_000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}:{:02}", mins, secs)
    }

    /// Get color for an event type
    fn event_color(&self, event: &RecordedEvent) -> Color {
        match event {
            RecordedEvent::Click(_)
            | RecordedEvent::DoubleClick(_)
            | RecordedEvent::MouseDown(_)
            | RecordedEvent::MouseUp(_)
            | RecordedEvent::MouseMove(_) => DebuggerColors::EVENT_MOUSE,
            RecordedEvent::KeyDown(_)
            | RecordedEvent::KeyUp(_)
            | RecordedEvent::TextInput(_) => DebuggerColors::EVENT_KEYBOARD,
            RecordedEvent::Scroll(_) => DebuggerColors::EVENT_SCROLL,
            RecordedEvent::FocusChange(_) => DebuggerColors::EVENT_FOCUS,
            RecordedEvent::HoverEnter(_) | RecordedEvent::HoverLeave(_) => {
                DebuggerColors::EVENT_HOVER
            }
            _ => DebuggerColors::TEXT_MUTED,
        }
    }
}

impl<'a> ElementBuilder for TimelinePanel<'a> {
    fn build_element(self) -> blinc_layout::element::Element {
        self.build().build_element()
    }
}
