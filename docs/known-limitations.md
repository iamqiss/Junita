# Known Limitations

This document describes known limitations in the Junita rendering system and their workarounds.

## Asymmetric Borders with Rounded Corners

**Status**: Known limitation
**Affected Components**: Any element with per-side border widths and rounded corners
**Workaround**: Use smaller corner radii (e.g., `RadiusToken::Sm` instead of `RadiusToken::Md`)

### Description

When an element has asymmetric border widths (e.g., 1px on sides, 3px on bottom) combined with rounded corners, visual artifacts may appear at the corners where borders of different widths meet.

### Technical Background

The border rendering uses SDF (Signed Distance Field) techniques. For uniform borders, the inner shape is simply the outer shape inset by the border width, with corner radii reduced accordingly. This works perfectly because the inner corners remain concentric circles.

For asymmetric borders, the geometry is more complex:
- The inner shape is offset differently on each side
- At corners, the inner edge should theoretically be an **ellipse**, not a circle
- The ellipse radii would be `(outer_radius - horizontal_border, outer_radius - vertical_border)`

### What We Tried

1. **GPUI-style elliptical inner corners**: Implemented `quarter_ellipse_sdf()` function following Zed's GPUI approach, which uses elliptical inner corners for asymmetric borders. This improved some cases but didn't fully resolve the artifacts.

2. **Per-region corner-aware SDF**: Attempted to calculate different SDFs for corner regions vs edge regions with smooth blending. This caused regressions (corners disappearing).

3. **Minimum inner radius**: Added `min_inner_r = 0.25` to prevent degenerate corners when border width approaches or exceeds corner radius.

### Current Implementation

The shader uses a hybrid approach based on GPUI:
- Quadrant-based border selection (picks relevant border widths per pixel)
- Fast path for pixels clearly inside the inner area
- Elliptical inner SDF for corners with asymmetric borders
- Circular inner SDF for corners with equal adjacent borders

### Workaround

Use smaller corner radii when asymmetric borders are needed. The artifacts become less noticeable (or invisible) when:
- Corner radius is small relative to the border difference
- Border width differences are minimal

**Example** (KBD component):
```rust
// Instead of RadiusToken::Md (6px) with 1px/3px borders
let radius = theme.radius(RadiusToken::Sm);  // 4px - artifacts less visible
```

### Browser Comparison

Web browsers handle this differently:
- Borders are rendered as **separate trapezoid geometry** per side
- Corners use **diagonal splits** where adjacent borders meet
- Each border side is painted independently ON TOP of the background

This approach handles asymmetric borders perfectly but requires more complex geometry generation. Our SDF approach trades this complexity for GPU efficiency but has limitations with edge cases.

### Future Improvements

Potential solutions that could be explored:

1. **Geometry-based borders**: Generate separate triangle geometry for each border side with diagonal corner splits (browser approach)

2. **Stencil-based approach**: Render inner shape to stencil, then draw borders as separate primitives

3. **Multi-pass rendering**: Render each border side separately with proper corner masking

4. **Improved ellipse SDF**: More accurate ellipse distance calculation that handles the transition between curved and straight regions better

### References

- [Zed GPUI shaders.wgsl](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform/blade/shaders.wgsl) - GPUI's border implementation
- [Leveraging Rust and the GPU to render user interfaces at 120 FPS](https://zed.dev/blog/videogame) - Zed's rendering approach
- [Drawing Rounded Corners and Borders with SDF](https://medium.com/@solidalloy/drawing-rounded-corners-and-borders-with-sdf-part-2-borders-1e7cf22bd571) - SDF border techniques
