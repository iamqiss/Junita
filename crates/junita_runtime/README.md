# junita_runtime

> **Part of the [Junita UI Framework](https://project-junita.github.io/Junita)**
>
> This crate is a component of Junita, a GPU-accelerated UI framework for Rust.
> For full documentation and guides, visit the [Junita documentation](https://project-junita.github.io/Junita).

Core runtime for Junita UI applications.

## Overview

`junita_runtime` is the embedding SDK for integrating Junita into Rust applications. It re-exports the essential crates needed for building Junita applications.

## Features

- **Modular**: Enable only the features you need
- **Full Feature**: `full` feature enables all components
- **Prelude**: Convenient re-exports for common usage

## Quick Start

```toml
[dependencies]
junita_runtime = { version = "0.1", features = ["full"] }
```

```rust
use junita_runtime::prelude::*;

fn main() {
    // Initialize runtime
    junita_runtime::init();

    // Build your UI
    let ui = div()
        .w_full()
        .h_full()
        .child(text("Hello from Junita Runtime!"));
}
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `full` | Enable all features | No |
| `core` | Core reactivity and events | Yes |
| `animation` | Animation system | Yes |
| `layout` | Layout engine | Yes |
| `gpu` | GPU rendering | No |
| `paint` | 2D painting API | Yes |

## Re-exports

When using the `full` feature:

```rust
// Core types
pub use junita_core::*;

// Animation
pub use junita_animation::*;

// Layout
pub use junita_layout::*;

// GPU (with gpu feature)
pub use junita_gpu::*;

// Paint
pub use junita_paint::*;
```

## Initialization

```rust
use junita_runtime;

fn main() {
    // Initialize global state, font registry, etc.
    junita_runtime::init();

    // Your application code
}
```

## Use Cases

- **Embedding**: Integrate Junita UI into existing Rust applications
- **Custom Shells**: Build custom application shells around Junita
- **Testing**: Create test harnesses for Junita components
- **Headless**: Render Junita UI without a window

## License

MIT OR Apache-2.0
