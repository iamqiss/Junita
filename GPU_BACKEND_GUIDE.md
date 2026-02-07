# GpuBackend Implementation Guide

This document explains the new `WidgetBackend` implementation of the `GpuBackend` trait and how it integrates with the hot reload system.

## Overview

The `WidgetBackend` provides a thread-safe, GPU-agnostic abstraction for tracking widget changes during hot reload. It bridges the high-level widget model used by the hot reload system with the low-level primitive rendering pipeline in `junita_gpu`.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│           Hot Reload Manager (junita_cli)               │
│                                                         │
│  - Watches file changes                                │
│  - Compiles .junita files                              │
│  - Compares old/new AST                                │
│  - Generates widget diffs                              │
└────────────────────┬────────────────────────────────────┘
                     │ WidgetDiff
                     ↓
┌─────────────────────────────────────────────────────────┐
│        RenderingAdapter (junita_core)                   │
│                                                         │
│  - Applies diffs to scene graph                         │
│  - Calls GpuBackend methods                             │
│  - Tracks hierarchy                                     │
└────────────────────┬────────────────────────────────────┘
                     │ GpuBackend trait calls
                     ↓
┌─────────────────────────────────────────────────────────┐
│     WidgetBackend (junita_gpu::gpu_backend)            │
│                                                         │
│  - Arc<Mutex<Box<dyn GpuBackend>>>                      │
│  - Tracks widgets: HashMap<u32, WidgetInfo>            │
│  - Marks frame dirty when changes occur                 │
│  - Thread-safe (Send + Sync)                            │
└────────────────────┬────────────────────────────────────┘
                     │ Future: Generate PrimitiveBatch
                     ↓
        ┌──────────────────────────┐
        │  junita_gpu::GpuRenderer  │
        │                          │
        │  - Low-level rendering   │
        │  - SDF primitives        │
        │  - Text/Images/Paths     │
        └──────────────────────────┘
```

## WidgetBackend Implementation

Located in: [crates/junita_gpu/src/gpu_backend.rs](crates/junita_gpu/src/gpu_backend.rs)

### Public API

```rust
impl WidgetBackend {
    /// Create a new widget backend
    pub fn new() -> Self { ... }

    /// Get widget registry statistics
    pub fn stats(&self) -> WidgetRegistryStats { ... }

    /// Mark frame as needing re-render
    pub fn is_frame_dirty(&self) -> bool { ... }
    pub fn clear_frame_dirty(&mut self) { ... }

    /// Access widget registry for debugging
    pub fn all_widgets(&self) -> Vec<WidgetInfo> { ... }

    /// Validate scene graph structure
    pub fn validate_hierarchy(&self) -> Result<()> { ... }
}
```

### GpuBackend Trait Implementation

```rust
impl GpuBackend for WidgetBackend {
    fn create_widget(&mut self, id: u32, widget_type: &str) -> Result<()> {
        // Register new widget in scene graph
        // Mark frame dirty
    }

    fn update_widget_properties(
        &mut self,
        id: u32,
        props: &HashMap<String, String>,
    ) -> Result<()> {
        // Update widget properties
        // Mark frame dirty
    }

    fn destroy_widget(&mut self, id: u32) -> Result<()> {
        // Remove widget and clean up hierarchy
        // Mark frame dirty
    }

    fn request_frame(&self) -> Result<()> {
        // Signal that frame re-render is needed
        // (currently a no-op; frame_dirty flag is used instead)
    }
}
```

## Integration with Hot Reload

### Data Flow

1. **File Change** → FileWatcher detects .junita file change
2. **Compilation** → JunitaCompiler parses and validates
3. **Diffing** → HotReloadManager compares AST trees
4. **Application** → RenderingAdapter applies diffs
5. **Backend** → WidgetBackend tracks scene graph
6. **Rendering** → Frame re-renders with updated widgets

### Example: Create Widget

```rust
// In hot_reload_manager::apply_diff()
WidgetDiff::Added { id, widget, parent_id } => {
    // Calls:
    adapter.add_widget_async(id.0, &widget.widget_type, parent_id.map(|p| p.0)).await?;
}

// In rendering_adapter::add_widget_async()
let mut backend = self.gpu_backend.lock().await;
backend.create_widget(id, widget_type)?;  // ← WidgetBackend method
```

### Example: Update Properties

```rust
// In hot_reload_manager::apply_diff()
WidgetDiff::Updated { id, changed_props } => {
    adapter.update_widget_properties_async(id.0, changed_props).await?;
}

// In rendering_adapter::update_widget_properties_async()
let mut backend = self.gpu_backend.lock().await;
backend.update_widget_properties(id, changed_props)?;  // ← WidgetBackend method
```

## Usage Example

### Creating a RenderingAdapter with WidgetBackend

```rust
use junita_gpu::WidgetBackend;
use junita_core::rendering::RenderingAdapter;
use std::sync::Arc;
use tokio::sync::Mutex;

// Create the backend
let backend = Box::new(WidgetBackend::new());
let backend = Arc::new(Mutex::new(backend));

// Create adapter with backend
let adapter = RenderingAdapter::with_gpu_backend(backend.clone());

// Now use adapter with hot reload system...
```

### Checking Widget State

```rust
// After applying diffs
let backend = backend.lock().await;

// Get statistics
let stats = backend.stats();
println!("Total widgets: {}", stats.total_widgets);
println!("Frame dirty: {}", stats.frame_dirty);

// Inspect all widgets
for widget in backend.all_widgets() {
    println!("Widget {}: {} ({:?} children)", widget.id, widget.widget_type, widget.children.len());
}

// Validate hierarchy
backend.validate_hierarchy()?;
```

## Widget Registry Data Structure

### WidgetInfo

```rust
pub struct WidgetInfo {
    pub id: u32,                               // Unique widget ID
    pub widget_type: String,                   // e.g., "button", "container"
    pub properties: HashMap<String, String>,   // Key-value properties
    pub children: Vec<u32>,                    // Child widget IDs
    pub parent_id: Option<u32>,               // Parent widget ID
}
```

### WidgetRegistryStats

```rust
pub struct WidgetRegistryStats {
    pub total_widgets: usize,         // Number of widgets
    pub root_id: Option<u32>,        // Top-level widget
    pub frame_dirty: bool,           // Needs re-render
}
```

## Thread Safety

The `WidgetBackend` is designed to be thread-safe:

- **Send + Sync**: Can be sent between threads and shared safely
- **Mutex Compatible**: Designed to be wrapped in `Arc<Mutex<>>`
- **No GPU Pointers**: Avoids holding raw GPU pointers that aren't Send

This makes it suitable for async hot reload pipelines:

```rust
// Safe to use in async code
let mut backend = gpu_backend.lock().await;
backend.create_widget(1, "button")?;
```

## Frame Dirty Flag

The frame dirty flag signals when re-rendering is needed:

```rust
// Set when changes occur
backend.create_widget(1, "button")?;      // Marks frame dirty
assert!(backend.is_frame_dirty());

// Clear after rendering
backend.clear_frame_dirty();
assert!(!backend.is_frame_dirty());
```

This allows the rendering pipeline to batch updates:

```rust
// Collect multiple diffs
for diff in diffs {
    adapter.apply_diff(&diff).await?;
}

// Check if re-render needed
if backend.is_frame_dirty() {
    // Trigger full frame re-render
    renderer.render(&target, &batch)?;
    backend.clear_frame_dirty();
}
```

## Error Handling

All GpuBackend methods return `Result<()>` for proper error handling:

```rust
// Error: Widget already exists
if backend.create_widget(1, "button").is_err() {
    println!("Widget 1 already exists!");
}

// Error: Widget not found
if backend.update_widget_properties(999, &props).is_err() {
    println!("Widget 999 not found!");
}

// Error: Invalid hierarchy
if backend.validate_hierarchy().is_err() {
    println!("Scene graph has structural issues!");
}
```

## Testing

### Unit Tests

The implementation includes comprehensive unit tests:

```bash
cargo test -p junita_gpu gpu_backend::tests
```

Tests cover:
- ✅ Widget creation and registration
- ✅ Property updates
- ✅ Widget destruction
- ✅ Duplicate creation errors
- ✅ Non-existent widget errors

### Integration with Hot Reload

To test integration with the full hot reload system:

1. Edit `examples/hot_reload_demo/main.junita`
2. Save the file
3. Watch as hot reload applies changes
4. Monitor backend state via logging

Example output:
```
[INFO] Created widget 1 of type 'Counter' (total widgets: 1)
[INFO] Updated widget 1 with 2 properties
[INFO] Frame render requested
```

## Future Enhancements

### Phase 2: PrimitiveBatch Generation

The next phase will extend `WidgetBackend` to generate GPU primitives:

```rust
impl WidgetBackend {
    /// Generate PrimitiveBatch from current widget state
    pub fn generate_batch(&self) -> Result<PrimitiveBatch> {
        // For each widget in scene graph:
        // - Determine layout (using junita_layout)
        // - Generate paint commands (using junita_paint)
        // - Build PrimitiveBatch for GPU rendering
    }

    /// Apply batch to GPU renderer
    pub async fn render(&mut self, renderer: &mut GpuRenderer) -> Result<()> {
        let batch = self.generate_batch()?;
        renderer.render(&target, &batch)?;
        self.clear_frame_dirty();
    }
}
```

### Phase 3: Layout Engine Integration

Connect the layout system for automatic position/size calculations:

```rust
pub async fn apply_diff(&mut self, diff: &WidgetDiff) -> Result<()> {
    // 1. Update scene graph
    self.apply_widget_change(diff)?;
    
    // 2. Recalculate layout
    let constraints = self.compute_constraints();
    let layout_tree = self.layout_engine.compute(&constraints)?;
    
    // 3. Update positions from layout
    self.apply_layout(&layout_tree)?;
    
    // 4. Mark for re-render
    self.mark_frame_dirty();
}
```

## Performance Considerations

### O(n) Widget Operations

All operations are O(n) in the number of widgets:
- Create: O(1) insert
- Update: O(1) lookup + update
- Destroy: O(n) for removing from children lists
- Validate: O(n) for checking hierarchy

For typical UIs with hundreds of widgets, this is negligible.

### Memory Usage

Widget registry memory:
- Per widget: ~120 bytes baseline
- Plus properties HashMap: varies
- Plus children Vec: varies

Example: 1000 widgets with average 3 properties each:
- ~120KB base + ~100KB properties + ~20KB children = ~250KB total

### Async Safety

Operations don't block:

```rust
// Thread-safe await
let mut backend = gpu_backend.lock().await;  // Quick lock
backend.create_widget(1, "button")?;          // Fast operation
drop(backend);                                 // Lock released
```

## Debugging and Inspection

### Logging

Enable logging to see backend operations:

```bash
RUST_LOG=debug cargo run --example hot_reload_demo
```

Output includes:
- Widget creation/destruction
- Property updates
- Frame dirty signals
- Hierarchy validation

### Statistics

Check backend statistics at any time:

```rust
let stats = backend.stats();
println!("Stats: {:#?}", stats);

// Output:
// WidgetRegistryStats {
//     total_widgets: 5,
//     root_id: Some(1),
//     frame_dirty: true,
// }
```

### Scene Graph Inspection

Debug the full hierarchy:

```rust
for widget in backend.all_widgets() {
    let indent = "  ".repeat(widget.children.len());
    println!(
        "{}Widget {}: {} with {} children",
        indent, widget.id, widget.widget_type, widget.children.len()
    );
    for (key, value) in &widget.properties {
        println!("{}  {}: {}", indent, key, value);
    }
}
```

## Comparison: MockGpuBackend vs WidgetBackend

| Feature | MockGpuBackend | WidgetBackend |
|---------|---|---|
| Purpose | Testing | Production |
| Thread-safe | ✅ Yes | ✅ Yes |
| Tracks state | ❌ No | ✅ Yes |
| Scene graph | ❌ No | ✅ Yes |
| Frame dirty | ❌ No | ✅ Yes |
| Validation | ❌ No | ✅ Yes |
| Size | Tiny | ~600 LOC |

## Migration from MockGpuBackend

To use `WidgetBackend` instead of the mock:

```rust
// Old: Use mock for testing
let adapter = RenderingAdapter::new();  // Uses MockGpuBackend internally

// New: Use real backend
let backend = Arc::new(Mutex::new(Box::new(WidgetBackend::new())));
let adapter = RenderingAdapter::with_gpu_backend(backend);
```

The API is identical; only the implementation differs.

## Conclusion

The `WidgetBackend` provides a solid foundation for hot reload integration with the GPU renderer. It:

- ✅ Tracks the complete widget scene graph
- ✅ Provides thread-safe access via Arc<Mutex>
- ✅ Signals frame re-renders via dirty flag
- ✅ Validates scene structure
- ✅ Enables debugging and inspection
- ✅ Scales efficiently for typical UIs

Future phases will extend it to generate GPU primitives and coordinate with the layout/paint systems for full end-to-end hot reload rendering.
