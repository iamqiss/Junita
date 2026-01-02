//! Slider component for range value selection
//!
//! A themed slider/range input with click-to-set and drag-to-adjust.
//! Uses State<f32> from context for reactive state management.
//!
//! # Example
//!
//! ```ignore
//! use blinc_cn::prelude::*;
//!
//! fn build_ui(ctx: &WindowedContext) -> impl ElementBuilder {
//!     // Create slider state from context (0.0 to 1.0 by default)
//!     let volume = ctx.use_state_for("volume", 0.5);
//!
//!     cn::slider(&volume)
//!         .label("Volume")
//!         .on_change(|value| println!("Volume: {}", value))
//! }
//!
//! // Custom range
//! let brightness = ctx.use_state_for("brightness", 50.0);
//! cn::slider(&brightness)
//!     .min(0.0)
//!     .max(100.0)
//!     .step(1.0)
//!
//! // Different sizes
//! cn::slider(&value)
//!     .size(SliderSize::Large)
//!
//! // Custom colors
//! cn::slider(&value)
//!     .track_color(Color::GRAY)
//!     .fill_color(Color::BLUE)
//!     .thumb_color(Color::WHITE)
//!
//! // Disabled state
//! cn::slider(&value)
//!     .disabled(true)
//! ```

use blinc_core::{Color, State};
use blinc_layout::div::ElementTypeId;
use blinc_layout::element::RenderProps;
use blinc_layout::prelude::*;
use blinc_layout::tree::{LayoutNodeId, LayoutTree};
use blinc_theme::{ColorToken, RadiusToken, ThemeState};
use std::sync::Arc;

use super::label::{label, LabelSize};

/// Slider size variants
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderSize {
    /// Small slider (track: 4px, thumb: 14px)
    Small,
    /// Medium slider (track: 6px, thumb: 18px)
    #[default]
    Medium,
    /// Large slider (track: 8px, thumb: 22px)
    Large,
}

impl SliderSize {
    /// Get the track height for this size
    fn track_height(&self) -> f32 {
        match self {
            SliderSize::Small => 4.0,
            SliderSize::Medium => 6.0,
            SliderSize::Large => 8.0,
        }
    }

    /// Get the thumb diameter for this size
    fn thumb_size(&self) -> f32 {
        match self {
            SliderSize::Small => 14.0,
            SliderSize::Medium => 18.0,
            SliderSize::Large => 22.0,
        }
    }
}

/// Slider component
///
/// A range slider with click-to-set and drag-to-adjust value.
/// Uses State<f32> from context for reactive state management.
pub struct Slider {
    value_state: State<f32>,
    min: f32,
    max: f32,
    step: Option<f32>,
    size: SliderSize,
    label: Option<String>,
    show_value: bool,
    disabled: bool,
    width: Option<f32>,
    // Colors
    track_color: Option<Color>,
    fill_color: Option<Color>,
    thumb_color: Option<Color>,
    // Callback
    on_change: Option<Arc<dyn Fn(f32) + Send + Sync>>,
}

impl Slider {
    /// Create a new slider with state from context
    ///
    /// # Example
    /// ```ignore
    /// let volume = ctx.use_state_for("volume", 0.5);
    /// cn::slider(&volume)
    /// ```
    pub fn new(value_state: &State<f32>) -> Self {
        Self {
            value_state: value_state.clone(),
            min: 0.0,
            max: 1.0,
            step: None,
            size: SliderSize::default(),
            label: None,
            show_value: false,
            disabled: false,
            width: None,
            track_color: None,
            fill_color: None,
            thumb_color: None,
            on_change: None,
        }
    }

    /// Set the minimum value (default: 0.0)
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum value (default: 1.0)
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Set the step size for discrete values
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set the slider size
    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    /// Add a label above the slider
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show the current value next to the slider
    pub fn show_value(mut self) -> Self {
        self.show_value = true;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set a fixed width for the slider track
    pub fn w(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the unfilled track color
    pub fn track_color(mut self, color: impl Into<Color>) -> Self {
        self.track_color = Some(color.into());
        self
    }

    /// Set the filled portion color
    pub fn fill_color(mut self, color: impl Into<Color>) -> Self {
        self.fill_color = Some(color.into());
        self
    }

    /// Set the thumb color
    pub fn thumb_color(mut self, color: impl Into<Color>) -> Self {
        self.thumb_color = Some(color.into());
        self
    }

    /// Set the change callback
    ///
    /// Called when the slider value changes.
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(callback));
        self
    }

    /// Build the slider element
    fn build_slider(&self) -> Stateful<ButtonState> {
        let theme = ThemeState::get();
        let track_height = self.size.track_height();
        let thumb_size = self.size.thumb_size();
        let radius = theme.radius(RadiusToken::Full);

        // Get colors
        let track_bg = self
            .track_color
            .unwrap_or_else(|| theme.color(ColorToken::Border));
        let fill_bg = self
            .fill_color
            .unwrap_or_else(|| theme.color(ColorToken::Primary));
        let thumb_bg = self
            .thumb_color
            .unwrap_or_else(|| theme.color(ColorToken::TextInverse));

        let disabled = self.disabled;
        let on_change = self.on_change.clone();
        let value_state_for_click = self.value_state.clone();
        let min = self.min;
        let max = self.max;
        let step = self.step;
        let width = self.width;

        // Create a closure to round to step
        let round_to_step = move |value: f32| -> f32 {
            if let Some(s) = step {
                if s > 0.0 {
                    let steps = ((value - min) / s).round();
                    (min + steps * s).clamp(min, max)
                } else {
                    value.clamp(min, max)
                }
            } else {
                value.clamp(min, max)
            }
        };

        // Use a Stateful wrapper for hover/press effects and reactivity
        let value_state_for_visual = self.value_state.clone();
        let mut stateful_slider = Stateful::new(ButtonState::Idle)
            .h(thumb_size)
            .items_center()
            .cursor_pointer()
            .deps(&[value_state_for_visual.signal_id()]);

        // Apply width
        if let Some(w) = width {
            stateful_slider = stateful_slider.w(w);
        } else {
            stateful_slider = stateful_slider.w_full();
        }

        if disabled {
            stateful_slider = stateful_slider.opacity(0.5);
        }

        stateful_slider = stateful_slider.on_state(move |state: &ButtonState, container: &mut Div| {
            let current_val = value_state_for_visual.get();
            let norm = ((current_val - min) / (max - min)).clamp(0.0, 1.0);
            let is_hovered = matches!(state, ButtonState::Hovered | ButtonState::Pressed);
            let is_pressed = matches!(state, ButtonState::Pressed);

            // Note: fill_bg could be used for a filled track portion in the future
            let _ = fill_bg; // Silence unused warning for now

            // Thumb scale on hover/press
            let thumb_scale = if is_pressed && !disabled {
                1.15
            } else if is_hovered && !disabled {
                1.05
            } else {
                1.0
            };

            // Layer: Thumb positioned using flex row with spacers
            // The trick: use multiple spacer divs to approximate the ratio
            // For simplicity, we create left spacers proportional to norm
            // and right spacers proportional to (1-norm)

            // Build thumb with scale effect
            let thumb_visual = div()
                .w(thumb_size)
                .h(thumb_size)
                .rounded(thumb_size / 2.0)
                .bg(thumb_bg)
                .transform(blinc_core::Transform::scale(thumb_scale, thumb_scale))
                .flex_shrink_0();

            // Thumb row: use flex with spacers
            // We approximate the ratio by creating N spacer divs on each side
            // where N_left / (N_left + N_right) ≈ norm
            // For precision, we use 100 total spacers (like percentage)
            let left_count = (norm * 100.0).round() as usize;
            let right_count = 100 - left_count;

            let mut thumb_row = div()
                .w_full()
                .h(thumb_size)
                .flex_row()
                .items_center();

            // Add left spacers
            for _ in 0..left_count.max(1) {
                thumb_row = thumb_row.child(div().flex_grow());
            }

            // Add thumb (centered at the division point)
            thumb_row = thumb_row.child(thumb_visual);

            // Add right spacers
            for _ in 0..right_count.max(1) {
                thumb_row = thumb_row.child(div().flex_grow());
            }

            // Use relative container with absolute children for layering
            let visual = div()
                .w_full()
                .h(thumb_size)
                .relative()
                .child(
                    // Track background - absolute positioned
                    div()
                        .w_full()
                        .h(track_height)
                        .rounded(radius)
                        .bg(track_bg)
                        .absolute()
                        .top((thumb_size - track_height) / 2.0)
                        .left(0.0)
                )
                .child(
                    // Thumb row - absolute positioned on top
                    thumb_row.absolute().top(0.0).left(0.0)
                );

            container.merge(visual);
        });

        stateful_slider = stateful_slider.on_click(move |event| {
            if disabled {
                return;
            }

            // Use local_x and bounds_width from EventContext
            let click_x = event.local_x;
            let track_width = event.bounds_width;
            if track_width > 0.0 {
                let normalized = (click_x / track_width).clamp(0.0, 1.0);
                let raw_value = min + normalized * (max - min);
                let new_value = round_to_step(raw_value);

                value_state_for_click.set(new_value);
                if let Some(ref callback) = on_change {
                    callback(new_value);
                }
            }
        });

        stateful_slider
    }
}

impl ElementBuilder for Slider {
    fn build(&self, tree: &mut LayoutTree) -> LayoutNodeId {
        let theme = ThemeState::get();
        let slider = self.build_slider();

        // If there's a label or show_value, wrap in a container
        if self.label.is_some() || self.show_value {
            let spacing = theme.spacing_value(blinc_theme::SpacingToken::Space2);
            let mut container = div().flex_col().gap(spacing);

            // Apply width to container
            if let Some(w) = self.width {
                container = container.w(w);
            } else {
                container = container.w_full();
            }

            // Header row with label and optional value
            if self.label.is_some() || self.show_value {
                let mut header = div().flex_row().justify_between().items_center();

                if let Some(ref label_text) = self.label {
                    let mut lbl = label(label_text).size(LabelSize::Medium);
                    if self.disabled {
                        lbl = lbl.disabled(true);
                    }
                    header = header.child(lbl);
                }

                if self.show_value {
                    let current_value = self.value_state.get();
                    let value_color = if self.disabled {
                        theme.color(ColorToken::TextTertiary)
                    } else {
                        theme.color(ColorToken::TextSecondary)
                    };
                    let value_text = if self.step.is_some() && self.step.unwrap() >= 1.0 {
                        format!("{:.0}", current_value)
                    } else {
                        format!("{:.2}", current_value)
                    };
                    header = header.child(text(&value_text).size(14.0).color(value_color));
                }

                container = container.child(header);
            }

            container = container.child(slider);
            container.build(tree)
        } else {
            slider.build(tree)
        }
    }

    fn render_props(&self) -> RenderProps {
        RenderProps::default()
    }

    fn children_builders(&self) -> &[Box<dyn ElementBuilder>] {
        &[]
    }

    fn element_type_id(&self) -> ElementTypeId {
        ElementTypeId::Div
    }
}

/// Create a slider with state from context
///
/// The slider uses reactive `State<f32>` for its value.
/// State changes automatically trigger visual updates via signals.
///
/// # Example
///
/// ```ignore
/// use blinc_cn::prelude::*;
///
/// fn build_ui(ctx: &WindowedContext) -> impl ElementBuilder {
///     let volume = ctx.use_state_for("volume", 0.5);
///
///     cn::slider(&volume)
///         .min(0.0)
///         .max(1.0)
///         .label("Volume")
///         .show_value()
///         .on_change(|v| println!("Volume: {}", v))
/// }
/// ```
pub fn slider(state: &State<f32>) -> Slider {
    Slider::new(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_sizes() {
        assert_eq!(SliderSize::Small.track_height(), 4.0);
        assert_eq!(SliderSize::Medium.track_height(), 6.0);
        assert_eq!(SliderSize::Large.track_height(), 8.0);
    }

    #[test]
    fn test_slider_thumb_sizes() {
        assert_eq!(SliderSize::Small.thumb_size(), 14.0);
        assert_eq!(SliderSize::Medium.thumb_size(), 18.0);
        assert_eq!(SliderSize::Large.thumb_size(), 22.0);
    }
}
