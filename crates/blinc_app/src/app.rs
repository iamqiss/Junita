//! Blinc Application Delegate
//!
//! The main entry point for Blinc applications.

use blinc_gpu::{GpuRenderer, RendererConfig, TextRenderingContext};
use blinc_layout::prelude::*;
use blinc_layout::RenderTree;
use std::sync::Arc;

use crate::context::RenderContext;
use crate::error::{BlincError, Result};

/// Blinc application configuration
#[derive(Clone, Debug)]
pub struct BlincConfig {
    /// Maximum primitives per batch
    pub max_primitives: usize,
    /// Maximum glass primitives per batch
    pub max_glass_primitives: usize,
    /// Maximum glyphs per batch
    pub max_glyphs: usize,
    /// MSAA sample count (1, 2, 4, or 8)
    pub sample_count: u32,
}

impl Default for BlincConfig {
    fn default() -> Self {
        Self {
            max_primitives: 10_000,
            max_glass_primitives: 1_000,
            max_glyphs: 50_000,
            sample_count: 4,
        }
    }
}

/// The main Blinc application
///
/// This is the primary interface for rendering Blinc UI.
/// It handles all GPU initialization and provides a clean API.
///
/// # Example
///
/// ```ignore
/// use blinc_app::prelude::*;
///
/// let app = BlincApp::new()?;
///
/// let ui = div()
///     .w(400.0).h(300.0)
///     .child(text("Hello!").size(24.0));
///
/// // Render to a texture
/// app.render(&ui, target_view, 400.0, 300.0)?;
/// ```
pub struct BlincApp {
    ctx: RenderContext,
    config: BlincConfig,
}

impl BlincApp {
    /// Create a new Blinc application with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(BlincConfig::default())
    }

    /// Create a new Blinc application with custom configuration
    pub fn with_config(config: BlincConfig) -> Result<Self> {
        let renderer_config = RendererConfig {
            max_primitives: config.max_primitives,
            max_glass_primitives: config.max_glass_primitives,
            max_glyphs: config.max_glyphs,
            sample_count: config.sample_count,
            texture_format: None,
        };

        let renderer = pollster::block_on(GpuRenderer::new(renderer_config))
            .map_err(|e| BlincError::GpuInit(e.to_string()))?;

        let device = renderer.device_arc();
        let queue = renderer.queue_arc();

        let mut text_ctx = TextRenderingContext::new(device.clone(), queue.clone());

        // Try to load a default system font
        #[cfg(target_os = "macos")]
        {
            let font_path = std::path::Path::new("/System/Library/Fonts/Helvetica.ttc");
            if font_path.exists() {
                if let Ok(data) = std::fs::read(font_path) {
                    let _ = text_ctx.load_font_data(data);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            let font_paths = [
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                "/usr/share/fonts/TTF/DejaVuSans.ttf",
            ];
            for path in &font_paths {
                if let Ok(data) = std::fs::read(path) {
                    let _ = text_ctx.load_font_data(data);
                    break;
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            let font_path = "C:\\Windows\\Fonts\\segoeui.ttf";
            if let Ok(data) = std::fs::read(font_path) {
                let _ = text_ctx.load_font_data(data);
            }
        }

        let ctx = RenderContext::new(renderer, text_ctx, device, queue, config.sample_count);

        Ok(Self { ctx, config })
    }

    /// Render a UI element tree to a texture
    ///
    /// This is the simplest way to render - just pass your UI tree
    /// and the target texture view.
    ///
    /// # Arguments
    ///
    /// * `element` - The root UI element (created with `div()`, etc.)
    /// * `target` - The texture view to render to
    /// * `width` - Viewport width in pixels
    /// * `height` - Viewport height in pixels
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ui = div().w(400.0).h(300.0)
    ///     .child(text("Hello!"));
    ///
    /// app.render(&ui, &target_view, 400.0, 300.0)?;
    /// ```
    pub fn render<E: ElementBuilder>(
        &mut self,
        element: &E,
        target: &wgpu::TextureView,
        width: f32,
        height: f32,
    ) -> Result<()> {
        let mut tree = RenderTree::from_element(element);
        tree.compute_layout(width, height);

        self.ctx.render_tree(
            &tree,
            width as u32,
            height as u32,
            target,
            None,
            None,
        )
    }

    /// Render with MSAA (multi-sample anti-aliasing)
    ///
    /// Use this for higher quality rendering with smooth edges.
    ///
    /// # Arguments
    ///
    /// * `element` - The root UI element
    /// * `msaa_target` - Multi-sampled texture view to render to
    /// * `resolve_target` - Single-sampled texture view for MSAA resolve
    /// * `width` - Viewport width
    /// * `height` - Viewport height
    pub fn render_msaa<E: ElementBuilder>(
        &mut self,
        element: &E,
        msaa_target: &wgpu::TextureView,
        resolve_target: &wgpu::TextureView,
        width: f32,
        height: f32,
    ) -> Result<()> {
        let mut tree = RenderTree::from_element(element);
        tree.compute_layout(width, height);

        self.ctx.render_tree(
            &tree,
            width as u32,
            height as u32,
            msaa_target,
            Some(resolve_target),
            None,
        )
    }

    /// Render with glass effects
    ///
    /// Use this when your UI contains glass elements that need backdrop blur.
    /// The backdrop texture should contain the content behind the glass.
    ///
    /// # Arguments
    ///
    /// * `element` - The root UI element
    /// * `target` - Texture view to render to
    /// * `backdrop` - Texture view containing backdrop for glass blur
    /// * `width` - Viewport width
    /// * `height` - Viewport height
    pub fn render_with_glass<E: ElementBuilder>(
        &mut self,
        element: &E,
        target: &wgpu::TextureView,
        backdrop: &wgpu::TextureView,
        width: f32,
        height: f32,
    ) -> Result<()> {
        let mut tree = RenderTree::from_element(element);
        tree.compute_layout(width, height);

        self.ctx.render_tree(
            &tree,
            width as u32,
            height as u32,
            target,
            None,
            Some(backdrop),
        )
    }

    /// Render a pre-computed render tree
    ///
    /// Use this when you want to compute layout once and render multiple times,
    /// or when you need more control over the render tree.
    pub fn render_tree(
        &mut self,
        tree: &RenderTree,
        target: &wgpu::TextureView,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.ctx.render_tree(tree, width, height, target, None, None)
    }

    /// Get the render context for advanced usage
    pub fn context(&mut self) -> &mut RenderContext {
        &mut self.ctx
    }

    /// Get the configuration
    pub fn config(&self) -> &BlincConfig {
        &self.config
    }

    /// Get the wgpu device
    pub fn device(&self) -> &Arc<wgpu::Device> {
        self.ctx.device()
    }

    /// Get the wgpu queue
    pub fn queue(&self) -> &Arc<wgpu::Queue> {
        self.ctx.queue()
    }
}
