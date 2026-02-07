//! Platform-agnostic context trait for Junita applications
//!
//! This module provides the `JunitaContext` trait which abstracts platform-specific
//! context implementations like `WindowedContext`. This enables components and
//! component libraries (like `junita_cn`) to be initialized without depending on
//! platform-specific code.
//!
//! # Architecture
//!
//! The context trait provides access to:
//! - **State Management**: `use_state`, `use_signal`, signals and derived values
//! - **Animations**: Access to the animation scheduler for spring/keyframe animations
//! - **Overlays**: Manager for modals, toasts, dropdowns, etc.
//! - **Refs**: Element references for programmatic control
//! - **Dirty Flag**: For triggering UI rebuilds
//!
//! # Example
//!
//! ```ignore
//! use junita_core::context::JunitaContext;
//!
//! fn my_component(ctx: &dyn JunitaContext) -> impl ElementBuilder {
//!     let count = ctx.use_state_keyed("count", || 0);
//!
//!     div()
//!         .child(text(&format!("Count: {}", count.get())))
//!         .on_click({
//!             let count = count.clone();
//!             move |_| count.set(count.get() + 1)
//!         })
//! }
//! ```

use crate::reactive::{Derived, DirtyFlag, ReactiveGraph, Signal, State};

/// Platform-agnostic context trait for Junita applications
///
/// This trait abstracts the platform-specific context (like `WindowedContext`)
/// and provides a common interface for:
/// - State management (signals, derived values, persistent state)
/// - Animation scheduling
/// - Overlay management
/// - Element references
///
/// # Thread Safety
///
/// Note that this trait does NOT require `Send + Sync`. The context is typically
/// owned by the main thread and accessed synchronously during UI builds. For
/// cross-thread access, use the shared handles like `SharedAnimationScheduler`,
/// `OverlayManager`, etc.
///
/// # Implementors
///
/// - `WindowedContext` (desktop/Android windowed apps)
/// - Future: `HeadlessContext` (testing), `WebContext` (WASM), etc.
pub trait JunitaContext {
    // =========================================================================
    // Reactive State Management
    // =========================================================================

    /// Create a persistent state value that survives across UI rebuilds (keyed)
    ///
    /// This creates component-level state identified by a unique string key.
    /// Returns a `State<T>` with direct `.get()` and `.set()` methods.
    fn use_state_keyed<T, F>(&self, key: &str, init: F) -> State<T>
    where
        T: Clone + Send + 'static,
        F: FnOnce() -> T;

    /// Create a persistent signal that survives across UI rebuilds (keyed)
    ///
    /// Unlike `use_signal()` which creates a new signal each call, this method
    /// persists the signal using a unique string key.
    fn use_signal_keyed<T, F>(&self, key: &str, init: F) -> Signal<T>
    where
        T: Clone + Send + 'static,
        F: FnOnce() -> T;

    /// Create a new reactive signal with an initial value (low-level API)
    ///
    /// **Note**: Prefer `use_state_keyed` in most cases, as it automatically
    /// persists signals across rebuilds.
    fn use_signal<T: Send + 'static>(&self, initial: T) -> Signal<T>;

    /// Get the current value of a signal
    fn get<T: Clone + 'static>(&self, signal: Signal<T>) -> Option<T>;

    /// Set the value of a signal, triggering reactive updates
    fn set<T: Send + 'static>(&self, signal: Signal<T>, value: T);

    /// Update a signal using a function
    fn update<T: Clone + Send + 'static, F: FnOnce(T) -> T>(&self, signal: Signal<T>, f: F);

    /// Create a derived (computed) value
    fn use_derived<T, F>(&self, compute: F) -> Derived<T>
    where
        T: Clone + Send + 'static,
        F: Fn(&ReactiveGraph) -> T + Send + 'static;

    /// Get the value of a derived computation
    fn get_derived<T: Clone + 'static>(&self, derived: Derived<T>) -> Option<T>;

    /// Batch multiple signal updates into a single reactive update
    fn batch<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut ReactiveGraph) -> R;

    // =========================================================================
    // Dirty Flag / Rebuild Triggering
    // =========================================================================

    /// Get the shared dirty flag for manual state management
    ///
    /// Use this when you want to create your own state types that trigger
    /// UI rebuilds when modified.
    fn dirty_flag(&self) -> DirtyFlag;

    /// Request a UI rebuild
    ///
    /// This is equivalent to setting the dirty flag to true.
    fn request_rebuild(&self);

    // =========================================================================
    // Window/Viewport Information
    // =========================================================================

    /// Get the current viewport width in logical pixels
    fn width(&self) -> f32;

    /// Get the current viewport height in logical pixels
    fn height(&self) -> f32;

    /// Get the current scale factor (physical / logical)
    fn scale_factor(&self) -> f64;
}

/// Extension trait for JunitaContext with additional convenience methods
///
/// This trait provides higher-level APIs built on top of the core JunitaContext trait.
pub trait JunitaContextExt: JunitaContext {
    /// Create a persistent state with automatic source-location key
    ///
    /// This is a convenience wrapper that uses `#[track_caller]` to automatically
    /// generate a unique key based on the call site.
    #[track_caller]
    fn use_state<T, F>(&self, init: F) -> State<T>
    where
        T: Clone + Send + 'static,
        F: FnOnce() -> T,
    {
        let location = std::panic::Location::caller();
        let key = format!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
        self.use_state_keyed(&key, init)
    }

    /// Create a persistent signal with automatic source-location key
    #[track_caller]
    fn use_signal_auto<T, F>(&self, init: F) -> Signal<T>
    where
        T: Clone + Send + 'static,
        F: FnOnce() -> T,
    {
        let location = std::panic::Location::caller();
        let key = format!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
        self.use_signal_keyed(&key, init)
    }
}

// Blanket implementation for all JunitaContext implementors
impl<T: JunitaContext + ?Sized> JunitaContextExt for T {}

#[cfg(test)]
mod tests {
    // Tests for JunitaContext trait are in integration tests
    // The trait is not dyn-compatible due to generic methods,
    // which is intentional - we use static dispatch for performance.
}
