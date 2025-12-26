//! Motion container for entry/exit animations
//!
//! A style-less container that applies animations to its children without
//! adding visual styling of its own.
//!
//! # Example
//!
//! ```ignore
//! use blinc_layout::prelude::*;
//!
//! // Single child with fade animation
//! motion()
//!     .fade_in(300)
//!     .fade_out(200)
//!     .child(my_content)
//!
//! // Stagger multiple children
//! motion()
//!     .stagger(StaggerConfig::new(50, AnimationPreset::fade_in(300)))
//!     .children(items.iter().map(|item| div().child(text(item))))
//! ```

use crate::div::ElementBuilder;
use crate::element::RenderProps;
use crate::tree::{LayoutNodeId, LayoutTree};
use blinc_animation::{AnimationPreset, MultiKeyframeAnimation};
use taffy::Style;

/// Animation configuration for element lifecycle
#[derive(Clone)]
pub struct ElementAnimation {
    /// The animation to play
    pub animation: MultiKeyframeAnimation,
}

impl ElementAnimation {
    /// Create a new element animation
    pub fn new(animation: MultiKeyframeAnimation) -> Self {
        Self { animation }
    }

    /// Set delay before animation starts
    pub fn with_delay(mut self, delay_ms: u32) -> Self {
        self.animation = self.animation.delay(delay_ms);
        self
    }
}

impl From<MultiKeyframeAnimation> for ElementAnimation {
    fn from(animation: MultiKeyframeAnimation) -> Self {
        Self::new(animation)
    }
}

/// Direction for stagger animations
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StaggerDirection {
    /// Animate first to last
    #[default]
    Forward,
    /// Animate last to first
    Reverse,
    /// Animate from center outward
    FromCenter,
}

/// Direction for slide animations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlideDirection {
    Left,
    Right,
    Top,
    Bottom,
}

/// Configuration for stagger animations
#[derive(Clone)]
pub struct StaggerConfig {
    /// Delay between each child's animation start (ms)
    pub delay_ms: u32,
    /// Animation to apply to each child
    pub animation: ElementAnimation,
    /// Direction of stagger
    pub direction: StaggerDirection,
    /// Optional: limit stagger to first N items
    pub limit: Option<usize>,
}

impl StaggerConfig {
    /// Create a new stagger config with delay between items
    pub fn new(delay_ms: u32, animation: impl Into<ElementAnimation>) -> Self {
        Self {
            delay_ms,
            animation: animation.into(),
            direction: StaggerDirection::Forward,
            limit: None,
        }
    }

    /// Stagger from last to first
    pub fn reverse(mut self) -> Self {
        self.direction = StaggerDirection::Reverse;
        self
    }

    /// Stagger from center outward
    pub fn from_center(mut self) -> Self {
        self.direction = StaggerDirection::FromCenter;
        self
    }

    /// Limit stagger to first N items
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Calculate delay for a specific child index
    pub fn delay_for_index(&self, index: usize, total: usize) -> u32 {
        let effective_index = match self.direction {
            StaggerDirection::Forward => index,
            StaggerDirection::Reverse => total.saturating_sub(1).saturating_sub(index),
            StaggerDirection::FromCenter => {
                let center = total / 2;
                if index <= center {
                    center - index
                } else {
                    index - center
                }
            }
        };

        // Apply limit if set
        let capped_index = if let Some(limit) = self.limit {
            effective_index.min(limit)
        } else {
            effective_index
        };

        self.delay_ms * capped_index as u32
    }
}

/// Style-less motion container for animations
///
/// Wraps child elements and applies entry/exit animations without
/// adding any visual styling of its own.
pub struct Motion {
    /// Single child element
    child: Option<Box<dyn ElementBuilder>>,
    /// Multiple children for stagger animations
    children: Vec<Box<dyn ElementBuilder>>,
    /// Entry animation
    enter: Option<ElementAnimation>,
    /// Exit animation
    exit: Option<ElementAnimation>,
    /// Stagger configuration for multiple children
    stagger_config: Option<StaggerConfig>,
}

/// Create a motion container
pub fn motion() -> Motion {
    Motion {
        child: None,
        children: Vec::new(),
        enter: None,
        exit: None,
        stagger_config: None,
    }
}

impl Motion {
    /// Set the child element to animate
    pub fn child(mut self, child: impl ElementBuilder + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    /// Add multiple children with stagger animation support
    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: ElementBuilder + 'static,
    {
        self.children = children
            .into_iter()
            .map(|c| Box::new(c) as Box<dyn ElementBuilder>)
            .collect();
        self
    }

    /// Set animation to play when element enters the tree
    pub fn enter_animation(mut self, animation: impl Into<ElementAnimation>) -> Self {
        self.enter = Some(animation.into());
        self
    }

    /// Set animation to play when element exits the tree
    pub fn exit_animation(mut self, animation: impl Into<ElementAnimation>) -> Self {
        self.exit = Some(animation.into());
        self
    }

    /// Enable stagger animations for multiple children
    pub fn stagger(mut self, config: StaggerConfig) -> Self {
        self.stagger_config = Some(config);
        self
    }

    // ========================================================================
    // Convenience methods for common animations
    // ========================================================================

    /// Fade in on enter
    pub fn fade_in(self, duration_ms: u32) -> Self {
        self.enter_animation(AnimationPreset::fade_in(duration_ms))
    }

    /// Fade out on exit
    pub fn fade_out(self, duration_ms: u32) -> Self {
        self.exit_animation(AnimationPreset::fade_out(duration_ms))
    }

    /// Scale in on enter
    pub fn scale_in(self, duration_ms: u32) -> Self {
        self.enter_animation(AnimationPreset::scale_in(duration_ms))
    }

    /// Scale out on exit
    pub fn scale_out(self, duration_ms: u32) -> Self {
        self.exit_animation(AnimationPreset::scale_out(duration_ms))
    }

    /// Bounce in on enter
    pub fn bounce_in(self, duration_ms: u32) -> Self {
        self.enter_animation(AnimationPreset::bounce_in(duration_ms))
    }

    /// Bounce out on exit
    pub fn bounce_out(self, duration_ms: u32) -> Self {
        self.exit_animation(AnimationPreset::bounce_out(duration_ms))
    }

    /// Slide in from direction
    pub fn slide_in(self, direction: SlideDirection, duration_ms: u32) -> Self {
        let distance = 50.0;
        let anim = match direction {
            SlideDirection::Left => AnimationPreset::slide_in_left(duration_ms, distance),
            SlideDirection::Right => AnimationPreset::slide_in_right(duration_ms, distance),
            SlideDirection::Top => AnimationPreset::slide_in_top(duration_ms, distance),
            SlideDirection::Bottom => AnimationPreset::slide_in_bottom(duration_ms, distance),
        };
        self.enter_animation(anim)
    }

    /// Slide out to direction
    pub fn slide_out(self, direction: SlideDirection, duration_ms: u32) -> Self {
        let distance = 50.0;
        let anim = match direction {
            SlideDirection::Left => AnimationPreset::slide_out_left(duration_ms, distance),
            SlideDirection::Right => AnimationPreset::slide_out_right(duration_ms, distance),
            SlideDirection::Top => AnimationPreset::slide_out_top(duration_ms, distance),
            SlideDirection::Bottom => AnimationPreset::slide_out_bottom(duration_ms, distance),
        };
        self.exit_animation(anim)
    }

    /// Pop in (scale with overshoot)
    pub fn pop_in(self, duration_ms: u32) -> Self {
        self.enter_animation(AnimationPreset::pop_in(duration_ms))
    }

    /// Get the enter animation if set
    pub fn get_enter_animation(&self) -> Option<&ElementAnimation> {
        self.enter.as_ref()
    }

    /// Get the exit animation if set
    pub fn get_exit_animation(&self) -> Option<&ElementAnimation> {
        self.exit.as_ref()
    }

    /// Get the stagger config if set
    pub fn get_stagger_config(&self) -> Option<&StaggerConfig> {
        self.stagger_config.as_ref()
    }

    /// Get all children (either from children vec or single child)
    fn all_children(&self) -> Vec<&Box<dyn ElementBuilder>> {
        if !self.children.is_empty() {
            self.children.iter().collect()
        } else if let Some(ref child) = self.child {
            vec![child]
        } else {
            vec![]
        }
    }
}

impl ElementBuilder for Motion {
    fn build(&self, tree: &mut LayoutTree) -> LayoutNodeId {
        // Create a transparent container node
        let node = tree.create_node(Style::default());

        // Build and add children
        // For stagger, we would store animation metadata on the nodes
        // For now, just build children normally
        if let Some(ref stagger) = self.stagger_config {
            let total = self.children.len();
            for (index, child) in self.children.iter().enumerate() {
                let child_node = child.build(tree);
                tree.add_child(node, child_node);
                // Calculate delay for this child (will be used by RenderState)
                let _delay = stagger.delay_for_index(index, total);
            }
        } else {
            for child in self.all_children() {
                let child_node = child.build(tree);
                tree.add_child(node, child_node);
            }
        }

        node
    }

    fn render_props(&self) -> RenderProps {
        // Motion container is transparent - no visual properties
        RenderProps::default()
    }

    fn children_builders(&self) -> &[Box<dyn ElementBuilder>] {
        // Return children vec if non-empty, otherwise empty slice
        // Note: single child is not returned here as it's boxed differently
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stagger_delay_forward() {
        let config = StaggerConfig::new(50, AnimationPreset::fade_in(300));

        assert_eq!(config.delay_for_index(0, 5), 0);
        assert_eq!(config.delay_for_index(1, 5), 50);
        assert_eq!(config.delay_for_index(2, 5), 100);
        assert_eq!(config.delay_for_index(4, 5), 200);
    }

    #[test]
    fn test_stagger_delay_reverse() {
        let config = StaggerConfig::new(50, AnimationPreset::fade_in(300)).reverse();

        assert_eq!(config.delay_for_index(0, 5), 200);
        assert_eq!(config.delay_for_index(1, 5), 150);
        assert_eq!(config.delay_for_index(4, 5), 0);
    }

    #[test]
    fn test_stagger_delay_from_center() {
        let config = StaggerConfig::new(50, AnimationPreset::fade_in(300)).from_center();

        // For 5 items, center is index 2
        // Distances from center: [2, 1, 0, 1, 2]
        assert_eq!(config.delay_for_index(0, 5), 100); // 2 steps from center
        assert_eq!(config.delay_for_index(1, 5), 50); // 1 step from center
        assert_eq!(config.delay_for_index(2, 5), 0); // at center
        assert_eq!(config.delay_for_index(3, 5), 50); // 1 step from center
        assert_eq!(config.delay_for_index(4, 5), 100); // 2 steps from center
    }

    #[test]
    fn test_stagger_delay_with_limit() {
        let config = StaggerConfig::new(50, AnimationPreset::fade_in(300)).limit(3);

        assert_eq!(config.delay_for_index(0, 10), 0);
        assert_eq!(config.delay_for_index(3, 10), 150); // capped at limit
        assert_eq!(config.delay_for_index(5, 10), 150); // still capped
        assert_eq!(config.delay_for_index(9, 10), 150); // still capped
    }
}
