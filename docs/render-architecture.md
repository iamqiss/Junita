# Blinc Render Architecture

This document describes the rendering architecture of the Blinc UI framework.

## Overview

Blinc uses a **SDF-based GPU rendering pipeline** that provides:
- Resolution-independent UI primitives with anti-aliasing
- Analytical shadows without texture lookups
- Glass/vibrancy effects (Apple-style backdrop blur)
- Unified 2D/3D rendering through dimension bridging
- Deferred command recording for efficient batching

```
User Code
    │
    ▼
┌──────────────────────────────┐
│ DrawContext API              │  ← High-level drawing API
│ (fill_rect, stroke_circle)   │
└────────┬─────────────────────┘
         │
         ▼
┌──────────────────────────────┐
│ RecordingContext             │  ← Command recording
│ (captures DrawCommand)       │
└────────┬─────────────────────┘
         │
         ▼
┌──────────────────────────────┐
│ GpuPaintContext              │  ← GPU translation
│ (emits GpuPrimitive)         │
└────────┬─────────────────────┘
         │
         ▼
┌──────────────────────────────┐
│ PrimitiveBatch               │  ← Batched GPU data
│ (primitives, glass, glyphs)  │
└────────┬─────────────────────┘
         │
         ▼
┌──────────────────────────────┐
│ GpuRenderer                  │  ← GPU execution
│ (SDF, Glass, Text pipelines) │
└────────┬─────────────────────┘
         │
         ▼
┌──────────────────────────────┐
│ Backbuffer                   │  ← Frame management
│ (double/triple buffering)    │
└────────┬─────────────────────┘
         │
         ▼
    Screen Output
```

## Core Components

### DrawContext Trait (`blinc_core::DrawContext`)

The unified rendering API that all drawing contexts implement:

```rust
pub trait DrawContext {
    // Transform stack
    fn push_transform(&mut self, transform: Transform);
    fn pop_transform(&mut self);

    // State stacks
    fn push_opacity(&mut self, opacity: f32);
    fn push_blend_mode(&mut self, mode: BlendMode);
    fn push_clip(&mut self, shape: ClipShape);

    // 2D drawing
    fn fill_rect(&mut self, rect: Rect, corner_radius: CornerRadius, brush: Brush);
    fn stroke_rect(&mut self, rect: Rect, corner_radius: CornerRadius, stroke: &Stroke, brush: Brush);
    fn fill_circle(&mut self, center: Point, radius: f32, brush: Brush);
    fn draw_shadow(&mut self, rect: Rect, corner_radius: CornerRadius, shadow: Shadow);
    fn draw_text(&mut self, text: &str, origin: Point, style: &TextStyle);

    // SDF builder for optimized UI
    fn sdf_build(&mut self, f: &mut dyn FnMut(&mut dyn SdfBuilder));

    // 3D operations
    fn set_camera(&mut self, camera: &Camera);
    fn draw_mesh(&mut self, mesh: MeshId, material: MaterialId, transform: Mat4);

    // Dimension bridging
    fn billboard_draw(&mut self, size: Size, transform: Mat4, facing: BillboardFacing, f: ...);
    fn viewport_3d_draw(&mut self, rect: Rect, camera: &Camera, f: ...);

    // Layer management
    fn push_layer(&mut self, config: LayerConfig);
    fn pop_layer(&mut self);
}
```

### RecordingContext

Records draw commands for deferred execution:

```rust
let mut ctx = RecordingContext::new(Size::new(800.0, 600.0));

ctx.push_transform(Transform::translate(10.0, 20.0));
ctx.fill_rect(Rect::new(0.0, 0.0, 100.0, 50.0), 8.0.into(), Color::BLUE.into());
ctx.pop_transform();

// Get recorded commands
let commands = ctx.take_commands();
```

### GpuPaintContext

Translates DrawContext commands into GPU primitives:

```rust
let mut gpu_ctx = GpuPaintContext::new(800.0, 600.0);

// Execute recorded commands
gpu_ctx.execute_commands(&commands);

// Get batched primitives for rendering
let batch = gpu_ctx.take_batch();
renderer.render(&target, &batch);
```

### PaintContext (`blinc_paint`)

Canvas-like convenience API that implements DrawContext:

```rust
let mut ctx = PaintContext::new(800.0, 600.0);

// Canvas-style API
ctx.fill_rect_xywh(10.0, 20.0, 100.0, 50.0, Color::BLUE);
ctx.translate(50.0, 50.0);
ctx.fill_rounded_rect_xywh(0.0, 0.0, 80.0, 40.0, 8.0, Color::RED);

// Commands are compatible with GPU execution
let commands = ctx.take_commands();
```

## Layer Model

All visual content is represented as composable layers:

```rust
pub enum Layer {
    // Content layers
    Ui { root: UiNode },
    Canvas2D { commands: Vec<DrawCommand> },
    Scene3D { camera: Camera, lights: Vec<Light>, ... },

    // Composition layers
    Stack { layers: Vec<LayerId>, blend_mode: BlendMode },
    Transform2D { transform: Affine2D, child: LayerId },
    Transform3D { transform: Mat4, child: LayerId },
    Clip { shape: ClipShape, child: LayerId },
    Opacity { opacity: f32, child: LayerId },
    Offscreen { child: LayerId, effects: Vec<PostEffect> },

    // Bridging layers
    Billboard { content: LayerId, transform: Mat4, facing: BillboardFacing },
    Viewport3D { scene: LayerId, camera: Camera },
    Portal { target: LayerId },
}
```

## GPU Primitives

### GpuPrimitive (144 bytes)

Single GPU-renderable primitive matching shader layout:

```rust
#[repr(C)]
pub struct GpuPrimitive {
    pub bounds: [f32; 4],           // x, y, width, height
    pub corner_radius: [f32; 4],    // per-corner radius
    pub color: [f32; 4],            // fill color (RGBA)
    pub color2: [f32; 4],           // gradient end color
    pub border: [f32; 4],           // border width, ...
    pub border_color: [f32; 4],     // border color
    pub shadow: [f32; 4],           // offset_x, offset_y, blur, spread
    pub shadow_color: [f32; 4],     // shadow color
    pub type_info: [u32; 4],        // primitive_type, fill_type, ...
}
```

### Primitive Types

- **Rect**: Rounded rectangle with per-corner radii
- **Circle**: Perfect circle
- **Ellipse**: Axis-aligned ellipse
- **Shadow**: Shadow-only primitive (no fill)

### Fill Types

- **Solid**: Single color fill
- **LinearGradient**: Interpolation along axis
- **RadialGradient**: Distance-based interpolation

## SDF Rendering

All UI primitives are rendered using Signed Distance Fields:

### Distance Functions

```wgsl
// Rounded rectangle SDF
fn sd_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let q = select(r.xy, r.zw, p.x > 0.0);
    let corner = select(q.x, q.y, p.y > 0.0);
    let d = abs(p) - b + corner;
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2(0.0))) - corner;
}

// Circle SDF
fn sd_circle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}
```

### Anti-Aliasing

Screen-space derivatives provide smooth edges:

```wgsl
let aa_width = fwidth(distance) * 0.5;
let alpha = smoothstep(aa_width, -aa_width, distance);
```

### Analytical Shadows

Gaussian blur shadows computed using error function:

```wgsl
fn erf_approx(x: f32) -> f32 {
    let t = 1.0 / (1.0 + 0.3275911 * abs(x));
    let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 +
               t * (-1.453152027 + t * 1.061405429))));
    return sign(x) * (1.0 - poly * exp(-x * x));
}

fn gaussian_shadow_2d(p: vec2<f32>, size: vec2<f32>, sigma: f32) -> f32 {
    let inv_sqrt2_sigma = 1.0 / (sqrt(2.0) * sigma);
    let x_integral = erf_approx((p.x + size.x) * inv_sqrt2_sigma) -
                     erf_approx((p.x - size.x) * inv_sqrt2_sigma);
    let y_integral = erf_approx((p.y + size.y) * inv_sqrt2_sigma) -
                     erf_approx((p.y - size.y) * inv_sqrt2_sigma);
    return x_integral * y_integral * 0.25;
}
```

## Glass/Vibrancy Effects

Apple-style frosted glass rendering:

### Glass Types

| Type | Blur | Saturation | Use Case |
|------|------|------------|----------|
| UltraThin | 10px | 1.8x | Subtle overlays |
| Thin | 15px | 1.6x | Light panels |
| Regular | 20px | 1.4x | Default glass |
| Thick | 30px | 1.2x | Strong blur |
| Chrome | 25px | 0.0x | Metallic effect |

### Glass Pipeline

1. **Shape Mask**: SDF-based clipping
2. **Backdrop Blur**: Kawase blur (efficient multi-pass)
3. **Saturation**: Luminance-based color adjustment
4. **Brightness**: RGB multiplier
5. **Noise**: Procedural frosted texture
6. **Tint**: Color overlay

```rust
let glass = GpuGlassPrimitive {
    bounds: [x, y, width, height],
    corner_radius: [8.0, 8.0, 8.0, 8.0],
    tint_color: [1.0, 1.0, 1.0, 0.1],
    params: [blur_radius, saturation, brightness, noise],
    type_info: [GlassType::Regular as u32, 0, 0, 0],
};
```

## Backbuffer System

Double/triple buffering for compositing effects:

```
Frame N-1          Frame N
┌─────────┐       ┌─────────┐
│  Read   │       │  Write  │
│ Target  │ ───── │ Target  │
└─────────┘       └─────────┘
     │                 │
     ▼                 ▼
Glass samples    Current render
  backdrop         output
```

### Use Cases

- Glass effects (need previous frame as backdrop)
- WASM targets (can't read from swapchain)
- Post-processing pipelines
- Screenshot capture

## Transform Composition

Transforms are composed hierarchically:

```rust
ctx.push_transform(Transform::translate(100.0, 50.0));  // T1
ctx.push_transform(Transform::scale(2.0, 2.0));         // T2
ctx.push_transform(Transform::rotate(0.5));             // T3

// Shape is transformed by: T1.then(T2).then(T3)
ctx.fill_rect(Rect::new(0.0, 0.0, 50.0, 25.0), ...);

ctx.pop_transform();  // Remove T3
ctx.pop_transform();  // Remove T2
ctx.pop_transform();  // Remove T1
```

### Affine2D Matrix

```rust
pub struct Affine2D {
    // [a, b, c, d, tx, ty]
    // | a  c  tx |
    // | b  d  ty |
    // | 0  0   1 |
    pub elements: [f32; 6],
}

impl Affine2D {
    pub fn then(&self, other: &Affine2D) -> Affine2D {
        // Matrix multiplication for concatenation
    }
}
```

## Opacity Stacking

Opacity values multiply through the stack:

```rust
ctx.push_opacity(0.5);           // Current: 0.5
ctx.push_opacity(0.5);           // Current: 0.25
ctx.fill_rect(...);              // Rendered at 25% opacity
ctx.pop_opacity();               // Current: 0.5
ctx.pop_opacity();               // Current: 1.0
```

## Performance Characteristics

### Batch Limits

| Resource | Max Count |
|----------|-----------|
| Primitives | 10,000 |
| Glass Primitives | 1,000 |
| Glyphs | 50,000 |

### Rendering Strategy

- **Instanced Rendering**: One draw call per primitive type
- **Storage Buffers**: Large instance data in GPU memory
- **No Tessellation**: SDF evaluation in fragment shader
- **Analytical Effects**: Shadows computed mathematically

### MSAA Support

Configurable multi-sample anti-aliasing: 1x, 2x, 4x

## File Structure

```
crates/
├── blinc_core/src/
│   ├── draw.rs        # DrawContext trait, RecordingContext
│   └── layer.rs       # Layer model, geometry types
├── blinc_gpu/src/
│   ├── renderer.rs    # GpuRenderer (wgpu pipelines)
│   ├── paint.rs       # GpuPaintContext (DrawContext → GPU)
│   ├── primitives.rs  # GpuPrimitive, PrimitiveBatch
│   ├── shaders.rs     # WGSL shader source
│   └── backbuffer.rs  # Double buffering
└── blinc_paint/src/
    └── context.rs     # PaintContext (Canvas-like API)
```

## Type Relationships

```
DrawContext (trait)
  ├── RecordingContext   → records DrawCommand
  ├── GpuPaintContext    → emits GpuPrimitive
  └── PaintContext       → wraps RecordingContext

Layer (enum)
  ├── Ui, Canvas2D, Scene3D    → content
  ├── Stack, Transform, Clip   → composition
  └── Billboard, Viewport3D    → bridging

GpuPrimitive
  └── rendered by SDF shader

GpuGlassPrimitive
  └── rendered by glass shader

PrimitiveBatch
  └── contains all GPU data for one frame
```
