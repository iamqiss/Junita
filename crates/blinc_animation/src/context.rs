//! Animation context trait for platform-agnostic animation management
//!
//! This module provides the `AnimationContext` trait which abstracts animation
//! scheduling and management from platform-specific implementations. This enables
//! components and component libraries to use animations without depending on
//! platform-specific code like `WindowedContext`.
//!
//! # Usage
//!
//! ```ignore
//! use blinc_animation::context::AnimationContext;
//! use blinc_animation::SpringConfig;
//!
//! fn animated_component(ctx: &impl AnimationContext) {
//!     // Create a persistent animated value
//!     let scale = ctx.use_animated_value(1.0, SpringConfig::snappy());
//!
//!     // Use in a motion binding
//!     motion().scale(scale.clone()).child(content)
//! }
//! ```
//!
//! # Combining with BlincContext
//!
//! For full context functionality, combine both traits:
//!
//! ```ignore
//! use blinc_core::BlincContext;
//! use blinc_animation::AnimationContext;
//!
//! fn my_component<C: BlincContext + AnimationContext>(ctx: &C) {
//!     let count = ctx.use_state_keyed("count", || 0);
//!     let scale = ctx.use_animated_value(1.0, SpringConfig::snappy());
//!     // ...
//! }
//! ```

use std::hash::Hash;
use std::sync::{Arc, Mutex};

use crate::scheduler::{AnimatedTimeline, AnimatedValue, SchedulerHandle};
use crate::spring::SpringConfig;

/// Shared animated value for persisting across UI rebuilds (thread-safe)
pub type SharedAnimatedValue = Arc<Mutex<AnimatedValue>>;

/// Shared animated timeline for persisting across UI rebuilds (thread-safe)
pub type SharedAnimatedTimeline = Arc<Mutex<AnimatedTimeline>>;

/// Platform-agnostic context trait for animation management
///
/// This trait abstracts the animation-related methods from platform-specific
/// contexts (like `WindowedContext`) and provides a common interface for:
/// - Creating and managing spring-based animated values
/// - Creating and managing keyframe timelines
/// - Accessing the animation scheduler
///
/// # Implementors
///
/// - `WindowedContext` (desktop/Android windowed apps)
/// - Future: `HeadlessContext` (testing), `WebContext` (WASM), etc.
pub trait AnimationContext {
    // =========================================================================
    // Animation Scheduler Access
    // =========================================================================

    /// Get a handle to the animation scheduler
    ///
    /// The scheduler handle is a weak reference that allows registering
    /// springs, keyframes, and timelines. It won't prevent the scheduler
    /// from being dropped.
    fn animation_handle(&self) -> SchedulerHandle;

    // =========================================================================
    // Persistent Animated Values
    // =========================================================================

    /// Create or retrieve a persistent animated value with an explicit key
    ///
    /// AnimatedValue provides spring-based physics animations that persist
    /// across UI rebuilds. Use this for values that need to survive layout
    /// changes and window resizes.
    ///
    /// # Arguments
    ///
    /// * `key` - A hashable key that uniquely identifies this animated value
    /// * `initial` - The initial value
    /// * `config` - Spring configuration (stiffness, damping, mass)
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Create with explicit key for loop scenarios
    /// for i in 0..3 {
    ///     let scale = ctx.use_animated_value_for(
    ///         format!("item_{}_scale", i),
    ///         1.0,
    ///         SpringConfig::snappy(),
    ///     );
    /// }
    /// ```
    fn use_animated_value_for<K: Hash>(
        &self,
        key: K,
        initial: f32,
        config: SpringConfig,
    ) -> SharedAnimatedValue;

    /// Create or retrieve a persistent animated timeline with an explicit key
    ///
    /// AnimatedTimeline provides keyframe-based animations that persist across
    /// UI rebuilds. Use this for timeline animations that need to survive
    /// layout changes and window resizes.
    ///
    /// The returned timeline is empty on first call - add keyframes using
    /// `timeline.add()` then call `start()`. Use `has_entries()` to check
    /// if the timeline needs configuration.
    ///
    /// # Arguments
    ///
    /// * `key` - A hashable key that uniquely identifies this timeline
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Create with explicit key for loop scenarios
    /// for i in 0..3 {
    ///     let timeline = ctx.use_animated_timeline_for(format!("dot_{}", i));
    ///     // Configure if needed...
    /// }
    /// ```
    fn use_animated_timeline_for<K: Hash>(&self, key: K) -> SharedAnimatedTimeline;
}

/// Extension trait for AnimationContext with convenience methods
///
/// This trait provides auto-keyed versions of animation methods that use
/// `#[track_caller]` to automatically generate unique keys based on call site.
pub trait AnimationContextExt: AnimationContext {
    /// Create or retrieve a persistent animated value (auto-keyed)
    ///
    /// Uses `#[track_caller]` to automatically generate a unique key based
    /// on the source location. For loop scenarios or reusable components,
    /// use `use_animated_value_for` with an explicit key instead.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Auto-keyed animated value
    /// let scale = ctx.use_animated_value(1.0, SpringConfig::snappy());
    ///
    /// // Set target to animate
    /// scale.lock().unwrap().set_target(1.2);
    /// ```
    #[track_caller]
    fn use_animated_value(&self, initial: f32, config: SpringConfig) -> SharedAnimatedValue {
        let location = std::panic::Location::caller();
        let key = format!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
        self.use_animated_value_for(&key, initial, config)
    }

    /// Create or retrieve a persistent animated timeline (auto-keyed)
    ///
    /// Uses `#[track_caller]` to automatically generate a unique key based
    /// on the source location. For loop scenarios or reusable components,
    /// use `use_animated_timeline_for` with an explicit key instead.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let timeline = ctx.use_animated_timeline();
    /// let entry_id = {
    ///     let mut t = timeline.lock().unwrap();
    ///     if !t.has_entries() {
    ///         let id = t.add(0, 2000, 0.0, 1.0);
    ///         t.start();
    ///         id
    ///     } else {
    ///         t.entry_ids().first().copied().unwrap()
    ///     }
    /// };
    /// ```
    #[track_caller]
    fn use_animated_timeline(&self) -> SharedAnimatedTimeline {
        let location = std::panic::Location::caller();
        let key = format!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
        self.use_animated_timeline_for(&key)
    }
}

// Blanket implementation for all AnimationContext implementors
impl<T: AnimationContext + ?Sized> AnimationContextExt for T {}

#[cfg(test)]
mod tests {
    // Tests for AnimationContext trait are in integration tests
    // The trait is not dyn-compatible due to generic methods,
    // which is intentional - we use static dispatch for performance.
}
