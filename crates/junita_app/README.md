# junita_app

> **Part of the [Junita UI Framework](https://project-junita.github.io/Junita)**
>
> This crate is a component of Junita, a GPU-accelerated UI framework for Rust.
> For full documentation and guides, visit the [Junita documentation](https://project-junita.github.io/Junita).

High-level application framework for Junita UI, combining layout and GPU rendering.

## Overview

`junita_app` provides the main entry point for building Junita applications. It integrates the layout engine with GPU rendering and provides both headless and windowed application modes.

## Features

- **Windowed Applications**: Native window support via winit
- **Headless Rendering**: Render to texture without a window
- **Text Rendering**: Integrated font loading and text measurement
- **Image Loading**: Async image loading with caching
- **Theme Integration**: Built-in theme support
- **Platform Abstraction**: Unified API across platforms

## Quick Start

### Windowed Application

```rust
use junita_app::prelude::*;
use junita_app::windowed::{WindowedApp, WindowConfig};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "My App".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, |ctx| build_ui(ctx))
}

fn build_ui(_ctx: &WindowedContext) -> impl ElementBuilder {
    div()
        .w_full()
        .h_full()
        .bg(Color::WHITE)
        .flex_col()
        .items_center()
        .justify_center()
        .child(
            text("Hello, Junita!")
                .size(48.0)
                .weight(FontWeight::Bold)
                .color(Color::BLACK)
        )
}
```

### Headless Rendering

```rust
use junita_app::{JunitaApp, RenderContext};

fn main() {
    let app = JunitaApp::new_headless(800, 600);

    // Build UI
    let ui = div()
        .w_full()
        .h_full()
        .bg(Color::WHITE)
        .child(text("Rendered headlessly"));

    // Render to texture
    let frame = app.render(&ui);

    // Save as image
    frame.save_png("output.png");
}
```

## Window Configuration

```rust
let config = WindowConfig {
    title: "My App".to_string(),
    width: 1024,
    height: 768,
    min_width: Some(400),
    min_height: Some(300),
    max_width: None,
    max_height: None,
    resizable: true,
    decorations: true,
    transparent: false,
    always_on_top: false,
    ..Default::default()
};
```

## Font Loading

```rust
use junita_app::system_font_paths;

// Get system font directories
let font_paths = system_font_paths();

// Load fonts
for path in font_paths {
    app.load_font_directory(&path);
}
```

## Architecture

```
junita_app
├── lib.rs           # Main JunitaApp type
├── context.rs       # RenderContext implementation
├── windowed.rs      # WindowedApp for native windows
├── headless.rs      # Headless rendering mode
└── prelude.rs       # Common re-exports
```

## Re-exports

`junita_app` re-exports commonly used types from:

- `junita_layout` - Layout primitives and elements
- `junita_core` - Core types (Color, Rect, etc.)
- `junita_gpu` - GPU renderer types

## Examples

See the `examples/` directory for complete examples:

- `hello_world.rs` - Basic windowed app
- `cn_demo.rs` - Component library showcase
- `image_layer_test.rs` - Image rendering test
- `glass_demo.rs` - Glass/blur effects

Run examples with:

```bash
cargo run -p junita_app --example hello_world --features windowed
```

## License

MIT OR Apache-2.0
