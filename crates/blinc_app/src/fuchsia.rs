//! Fuchsia application runner
//!
//! Provides a unified API for running Blinc applications on Fuchsia OS.
//!
//! # Example
//!
//! ```ignore
//! use blinc_app::prelude::*;
//! use blinc_app::fuchsia::FuchsiaApp;
//!
//! #[no_mangle]
//! fn main() {
//!     FuchsiaApp::run(|ctx| {
//!         div().w(ctx.width).h(ctx.height)
//!             .bg([0.1, 0.1, 0.15, 1.0])
//!             .flex_center()
//!             .child(text("Hello Fuchsia!").size(48.0))
//!     }).unwrap();
//! }
//! ```
//!
//! # Architecture
//!
//! Fuchsia applications integrate with the system through:
//!
//! - **Scenic/Flatland** - Window compositing via Views
//! - **fuchsia-async** - Async executor for event handling
//! - **FIDL** - IPC with system services
//! - **Vulkan** - GPU rendering via ImagePipe2
//!
//! # Building
//!
//! Requires the Fuchsia SDK and target:
//!
//! ```bash
//! rustup target add x86_64-unknown-fuchsia
//! cargo build --target x86_64-unknown-fuchsia --features fuchsia
//! ```

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use blinc_animation::AnimationScheduler;
use blinc_core::context_state::{BlincContextState, HookState, SharedHookState};
use blinc_core::reactive::{ReactiveGraph, SignalId};
use blinc_layout::event_router::MouseButton;
use blinc_layout::overlay_state::OverlayContext;
use blinc_layout::prelude::*;
use blinc_layout::widgets::overlay::{overlay_manager, OverlayManager};
use blinc_platform::assets::set_global_asset_loader;
use blinc_platform_fuchsia::{FuchsiaAssetLoader, FuchsiaPlatform, FuchsiaWakeProxy};

use crate::app::BlincApp;
use crate::error::{BlincError, Result};
use crate::windowed::{
    RefDirtyFlag, SharedAnimationScheduler, SharedElementRegistry, SharedReactiveGraph,
    SharedReadyCallbacks, WindowedContext,
};

/// Fuchsia application runner
///
/// Provides a simple way to run a Blinc application on Fuchsia OS
/// with automatic event handling and rendering via Scenic.
pub struct FuchsiaApp;

impl FuchsiaApp {
    /// Initialize the Fuchsia asset loader
    fn init_asset_loader() {
        let loader = FuchsiaAssetLoader::new();
        let _ = set_global_asset_loader(Box::new(loader));
    }

    /// Initialize the theme system
    fn init_theme() {
        use blinc_theme::{
            detect_system_color_scheme, platform_theme_bundle, set_redraw_callback, ThemeState,
        };

        // Only initialize if not already initialized
        if ThemeState::try_get().is_none() {
            let bundle = platform_theme_bundle();
            let scheme = detect_system_color_scheme();
            ThemeState::init(bundle, scheme);
        }

        // Set up the redraw callback
        set_redraw_callback(|| {
            tracing::debug!("Theme changed - requesting full rebuild");
            blinc_layout::widgets::request_full_rebuild();
        });
    }

    /// Initialize logging for Fuchsia
    fn init_logging() {
        // Fuchsia uses syslog - set up tracing subscriber
        use tracing_subscriber::layer::SubscriberExt;
        let subscriber = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_target(true));
        let _ = tracing::subscriber::set_global_default(subscriber);
    }

    /// Run a Fuchsia Blinc application
    ///
    /// This is the main entry point for Fuchsia applications. It sets up
    /// the GPU renderer via Scenic, handles lifecycle events, and runs the event loop.
    ///
    /// # Arguments
    ///
    /// * `ui_builder` - Function that builds the UI tree given the window context
    ///
    /// # Example
    ///
    /// ```ignore
    /// FuchsiaApp::run(|ctx| {
    ///     div()
    ///         .w(ctx.width).h(ctx.height)
    ///         .bg([0.1, 0.1, 0.15, 1.0])
    ///         .flex_center()
    ///         .child(text("Hello Fuchsia!").size(32.0))
    /// })
    /// ```
    #[cfg(target_os = "fuchsia")]
    pub fn run<F, E>(mut ui_builder: F) -> Result<()>
    where
        F: FnMut(&mut WindowedContext) -> E + 'static,
        E: ElementBuilder + 'static,
    {
        // Initialize logging first
        Self::init_logging();
        tracing::info!("FuchsiaApp::run starting");

        // Initialize the asset loader
        Self::init_asset_loader();

        // Initialize the text measurer
        crate::text_measurer::init_text_measurer();

        // Initialize the theme system
        Self::init_theme();

        // Shared state
        let ref_dirty_flag: RefDirtyFlag = Arc::new(AtomicBool::new(false));
        let reactive: SharedReactiveGraph = Arc::new(Mutex::new(ReactiveGraph::new()));
        let hooks: SharedHookState = Arc::new(Mutex::new(HookState::new()));

        // Initialize global context state singleton
        if !BlincContextState::is_initialized() {
            let stateful_callback: Arc<dyn Fn(&[SignalId]) + Send + Sync> =
                Arc::new(|signal_ids| {
                    blinc_layout::check_stateful_deps(signal_ids);
                });
            BlincContextState::init_with_callback(
                Arc::clone(&reactive),
                Arc::clone(&hooks),
                Arc::clone(&ref_dirty_flag),
                stateful_callback,
            );
        }

        // Animation scheduler
        let mut scheduler = AnimationScheduler::new();

        // Set up wake proxy for Fuchsia - allows animation thread to wake event loop
        let wake_proxy = FuchsiaWakeProxy::new();
        let wake_proxy_clone = wake_proxy.clone();
        scheduler.set_wake_callback(move || wake_proxy_clone.wake());
        tracing::info!("Fuchsia WakeProxy enabled for animations");

        scheduler.start_background();
        let animations: SharedAnimationScheduler = Arc::new(Mutex::new(scheduler));

        // Set global scheduler handle
        {
            let scheduler_handle = animations.lock().unwrap().handle();
            blinc_animation::set_global_scheduler(scheduler_handle);
        }

        // Element registry for query API
        let element_registry: SharedElementRegistry =
            Arc::new(blinc_layout::selector::ElementRegistry::new());

        // Set up query callback
        {
            let registry_for_query = Arc::clone(&element_registry);
            let query_callback: blinc_core::QueryCallback = Arc::new(move |id: &str| {
                registry_for_query.get(id).map(|node_id| node_id.to_raw())
            });
            BlincContextState::get().set_query_callback(query_callback);
        }

        // Set up bounds callback
        {
            let registry_for_bounds = Arc::clone(&element_registry);
            let bounds_callback: blinc_core::BoundsCallback =
                Arc::new(move |id: &str| registry_for_bounds.get_bounds(id));
            BlincContextState::get().set_bounds_callback(bounds_callback);
        }

        // Store element registry in BlincContextState
        BlincContextState::get()
            .set_element_registry(Arc::clone(&element_registry) as blinc_core::AnyElementRegistry);

        // Ready callbacks
        let ready_callbacks: SharedReadyCallbacks = Arc::new(Mutex::new(Vec::new()));

        // Overlay manager
        let overlays: OverlayManager = overlay_manager();
        if !OverlayContext::is_initialized() {
            OverlayContext::init(Arc::clone(&overlays));
        }

        // Connect theme animation to scheduler
        blinc_theme::ThemeState::get().set_scheduler(&animations);

        // Render state and motion states
        let shared_motion_states = blinc_layout::create_shared_motion_states();

        // Set up motion state callback
        {
            let motion_states_for_callback = Arc::clone(&shared_motion_states);
            let motion_callback: blinc_core::MotionStateCallback = Arc::new(move |key: &str| {
                motion_states_for_callback
                    .read()
                    .ok()
                    .and_then(|states| states.get(key).copied())
                    .unwrap_or(blinc_core::MotionAnimationState::NotFound)
            });
            BlincContextState::get().set_motion_state_callback(motion_callback);
        }

        // TODO: Connect to Scenic and run the event loop
        // This requires the actual Fuchsia SDK and FIDL bindings:
        //
        // 1. Connect to fuchsia.ui.scenic.Scenic
        // 2. Create a Session with Vulkan rendering
        // 3. Create a View via fuchsia.ui.views.View
        // 4. Subscribe to fuchsia.ui.pointer events for input
        // 5. Run fuchsia-async executor for event handling
        //
        // For now, this is a placeholder that will be filled in when
        // building with the actual Fuchsia SDK.

        tracing::warn!(
            "Fuchsia event loop not yet implemented - requires Fuchsia SDK FIDL bindings"
        );

        // Application state - these will be initialized when Scenic is connected
        let mut _blinc_app: Option<BlincApp> = None;
        let mut _surface: Option<wgpu::Surface<'static>> = None;
        let mut _surface_config: Option<wgpu::SurfaceConfiguration> = None;
        let mut _ctx: Option<WindowedContext> = None;
        let mut _render_tree: Option<RenderTree> = None;
        let mut _render_state: Option<blinc_layout::RenderState> = None;

        // Default window size for Fuchsia (will be updated from ViewProperties)
        let width = 1920u32;
        let height = 1080u32;
        let scale_factor = 1.0f64;
        let logical_width = width as f32 / scale_factor as f32;
        let logical_height = height as f32 / scale_factor as f32;

        // Create WindowedContext with default size
        let mut ctx = WindowedContext::new_fuchsia(
            logical_width,
            logical_height,
            scale_factor,
            width as f32,
            height as f32,
            true, // focused
            Arc::clone(&animations),
            Arc::clone(&ref_dirty_flag),
            Arc::clone(&reactive),
            Arc::clone(&hooks),
            Arc::clone(&overlays),
            Arc::clone(&element_registry),
            Arc::clone(&ready_callbacks),
        );

        // Set viewport size
        BlincContextState::get().set_viewport_size(logical_width, logical_height);

        // Build UI once to validate
        let _element = ui_builder(&mut ctx);
        tracing::info!("UI tree built successfully");

        tracing::info!("FuchsiaApp::run exiting (placeholder implementation)");
        Ok(())
    }

    /// Placeholder for non-Fuchsia builds
    #[cfg(not(target_os = "fuchsia"))]
    pub fn run<F, E>(_ui_builder: F) -> Result<()>
    where
        F: FnMut(&mut WindowedContext) -> E + 'static,
        E: ElementBuilder + 'static,
    {
        Err(BlincError::PlatformUnsupported(
            "Fuchsia apps can only run on Fuchsia OS".to_string(),
        ))
    }

    /// Get the system font paths for Fuchsia
    pub fn system_font_paths() -> &'static [&'static str] {
        FuchsiaPlatform::system_font_paths()
    }
}
