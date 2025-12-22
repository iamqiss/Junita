//! Render context for blinc_app
//!
//! Wraps the GPU rendering pipeline with a clean API.

use blinc_core::Rect;
use blinc_gpu::{GpuGlyph, GpuPaintContext, GpuRenderer, PrimitiveBatch, TextRenderingContext};
use blinc_layout::prelude::*;
use blinc_layout::renderer::ElementType;
use blinc_svg::SvgDocument;
use blinc_text::TextAnchor;
use std::sync::Arc;

use crate::error::Result;

/// Internal render context that manages GPU resources and rendering
pub struct RenderContext {
    renderer: GpuRenderer,
    text_ctx: TextRenderingContext,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    sample_count: u32,
}

impl RenderContext {
    /// Create a new render context
    pub(crate) fn new(
        renderer: GpuRenderer,
        text_ctx: TextRenderingContext,
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        sample_count: u32,
    ) -> Self {
        Self {
            renderer,
            text_ctx,
            device,
            queue,
            sample_count,
        }
    }

    /// Render a layout tree to a texture view
    ///
    /// This is the main rendering method that handles everything automatically:
    /// - Renders background layer
    /// - Renders glass layer with backdrop blur
    /// - Renders foreground layer on top
    /// - Renders text at layout-computed positions
    /// - Renders SVG elements at layout-computed positions
    pub fn render_tree(
        &mut self,
        tree: &RenderTree,
        width: u32,
        height: u32,
        target: &wgpu::TextureView,
        resolve_target: Option<&wgpu::TextureView>,
        backdrop: Option<&wgpu::TextureView>,
    ) -> Result<()> {
        // Create paint contexts for each layer
        let mut bg_ctx = GpuPaintContext::new(width as f32, height as f32);
        let mut fg_ctx = GpuPaintContext::new(width as f32, height as f32);

        // Render layout layers
        tree.render_to_layer(&mut bg_ctx, RenderLayer::Background);
        tree.render_to_layer(&mut bg_ctx, RenderLayer::Glass);
        tree.render_to_layer(&mut fg_ctx, RenderLayer::Foreground);

        // Collect text and SVG elements
        let (texts, svgs) = self.collect_render_elements(tree);

        // Prepare text glyphs
        let mut all_glyphs = Vec::new();
        for (content, x, y, _w, h, font_size, color) in &texts {
            if let Ok(glyphs) = self.text_ctx.prepare_text_with_anchor(
                content,
                *x,
                *y + *h / 2.0,
                *font_size,
                *color,
                TextAnchor::Center,
            ) {
                all_glyphs.extend(glyphs);
            }
        }

        // Render SVGs to foreground context
        for (source, x, y, w, h) in &svgs {
            if let Ok(doc) = SvgDocument::from_str(source) {
                doc.render_fit(&mut fg_ctx, Rect::new(*x, *y, *w, *h));
            }
        }

        // Take batches
        let bg_batch = bg_ctx.take_batch();
        let fg_batch = fg_ctx.take_batch();

        self.renderer.resize(width, height);

        // Render based on whether we have glass effects
        if bg_batch.glass_count() > 0 && backdrop.is_some() {
            // Multi-pass glass rendering
            self.render_with_glass(
                target,
                resolve_target,
                backdrop.unwrap(),
                &bg_batch,
                &fg_batch,
                &all_glyphs,
            )?;
        } else {
            // Simple rendering without glass
            self.render_simple(
                target,
                resolve_target,
                &bg_batch,
                &fg_batch,
                &all_glyphs,
            )?;
        }

        Ok(())
    }

    /// Simple render path (no glass effects)
    fn render_simple(
        &mut self,
        target: &wgpu::TextureView,
        resolve_target: Option<&wgpu::TextureView>,
        bg_batch: &PrimitiveBatch,
        fg_batch: &PrimitiveBatch,
        glyphs: &[GpuGlyph],
    ) -> Result<()> {
        // Render background
        if let Some(resolve) = resolve_target {
            self.renderer
                .render_msaa(target, resolve, bg_batch, [1.0, 1.0, 1.0, 1.0]);
        } else {
            self.renderer
                .render_with_clear(target, bg_batch, [1.0, 1.0, 1.0, 1.0]);
        }

        // Render foreground overlay
        let final_target = resolve_target.unwrap_or(target);
        if fg_batch.primitive_count() > 0 {
            self.renderer
                .render_overlay_msaa(final_target, fg_batch, self.sample_count);
        }

        // Render text
        if !glyphs.is_empty() {
            self.render_text(final_target, glyphs);
        }

        Ok(())
    }

    /// Multi-pass render with glass effects
    fn render_with_glass(
        &mut self,
        target: &wgpu::TextureView,
        resolve_target: Option<&wgpu::TextureView>,
        backdrop: &wgpu::TextureView,
        bg_batch: &PrimitiveBatch,
        fg_batch: &PrimitiveBatch,
        glyphs: &[GpuGlyph],
    ) -> Result<()> {
        let final_target = resolve_target.unwrap_or(target);

        // Step 1: Render background with MSAA
        if let Some(resolve) = resolve_target {
            self.renderer
                .render_msaa(target, resolve, bg_batch, [1.0, 1.0, 1.0, 1.0]);
        } else {
            self.renderer
                .render_with_clear(target, bg_batch, [1.0, 1.0, 1.0, 1.0]);
        }

        // Step 2: Copy to backdrop and render glass
        if bg_batch.glass_count() > 0 {
            // Copy current content to backdrop texture for glass sampling
            self.copy_texture(final_target, backdrop);

            // Render glass with backdrop blur
            self.renderer.render_glass(final_target, backdrop, bg_batch);
        }

        // Step 3: Render foreground on top of glass
        if fg_batch.primitive_count() > 0 {
            self.renderer
                .render_overlay_msaa(final_target, fg_batch, self.sample_count);
        }

        // Step 4: Render text
        if !glyphs.is_empty() {
            self.render_text(final_target, glyphs);
        }

        Ok(())
    }

    /// Copy texture contents
    fn copy_texture(&self, _src: &wgpu::TextureView, _dst: &wgpu::TextureView) {
        // Note: This is a simplified placeholder. In a real implementation,
        // we'd need access to the underlying textures, not just views.
        // The actual copy would be done via command encoder.
    }

    /// Render text glyphs
    fn render_text(&mut self, target: &wgpu::TextureView, glyphs: &[GpuGlyph]) {
        if let Some(atlas_view) = self.text_ctx.atlas_view() {
            self.renderer
                .render_text(target, glyphs, atlas_view, self.text_ctx.sampler());
        }
    }

    /// Collect text and SVG elements from the render tree
    fn collect_render_elements(
        &self,
        tree: &RenderTree,
    ) -> (
        Vec<(String, f32, f32, f32, f32, f32, [f32; 4])>,
        Vec<(String, f32, f32, f32, f32)>,
    ) {
        let mut texts = Vec::new();
        let mut svgs = Vec::new();

        if let Some(root) = tree.root() {
            self.collect_elements_recursive(tree, root, (0.0, 0.0), &mut texts, &mut svgs);
        }

        (texts, svgs)
    }

    fn collect_elements_recursive(
        &self,
        tree: &RenderTree,
        node: LayoutNodeId,
        parent_offset: (f32, f32),
        texts: &mut Vec<(String, f32, f32, f32, f32, f32, [f32; 4])>,
        svgs: &mut Vec<(String, f32, f32, f32, f32)>,
    ) {
        let Some(bounds) = tree.layout().get_bounds(node, parent_offset) else {
            return;
        };

        let abs_x = bounds.x;
        let abs_y = bounds.y;

        if let Some(render_node) = tree.get_render_node(node) {
            match &render_node.element_type {
                ElementType::Text(text_data) => {
                    texts.push((
                        text_data.content.clone(),
                        abs_x,
                        abs_y,
                        bounds.width,
                        bounds.height,
                        text_data.font_size,
                        text_data.color,
                    ));
                }
                ElementType::Svg(svg_data) => {
                    svgs.push((
                        svg_data.source.clone(),
                        abs_x,
                        abs_y,
                        bounds.width,
                        bounds.height,
                    ));
                }
                ElementType::Div => {}
            }
        }

        let new_offset = (abs_x, abs_y);
        for child_id in tree.layout().children(node) {
            self.collect_elements_recursive(tree, child_id, new_offset, texts, svgs);
        }
    }

    /// Get device arc
    pub fn device(&self) -> &Arc<wgpu::Device> {
        &self.device
    }

    /// Get queue arc
    pub fn queue(&self) -> &Arc<wgpu::Queue> {
        &self.queue
    }
}
