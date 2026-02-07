# Introduction

**Junita** is a GPU-accelerated, reactive UI framework for Rust. It provides a declarative, component-based approach to building high-performance user interfaces with smooth animations and modern visual effects.

## Why Junita?

- **GPU-Accelerated Rendering** - All rendering is done on the GPU via wgpu, enabling smooth 60fps animations and complex visual effects like glass materials and shadows.

- **Declarative UI** - Build interfaces using a fluent, composable API inspired by SwiftUI and modern web frameworks. No manual DOM manipulation.

- **Reactive State** - Automatic UI updates when state changes, with fine-grained reactivity for optimal performance.

- **Spring Physics** - Natural, physics-based animations using spring dynamics instead of fixed durations.

- **Cross-Platform** - Runs on macOS, Windows, Linux, and Android (iOS coming soon).

## Key Features

### Flexbox Layout
All layout is powered by [Taffy](https://github.com/DioxusLabs/taffy), a high-performance flexbox implementation. Use familiar CSS-like properties:

```rust
div()
    .flex_col()
    .gap(16.0)
    .p(24.0)
    .child(text("Hello"))
    .child(text("World"))
```

### Material Effects
Built-in support for glass, metallic, and other material effects:

```rust
```markdown
# Junita — The UI framework that actually feels alive

![window graphic](../../window.svg)

<div align="center">
  ![Junita logo](../../logo.svg){:height="64"}
</div>

Junita is a GPU-first, reactive UI framework for Rust — built for people who want silky animations, modern material effects, and the control of native rendering without the pain of low-level GPU plumbing.

Why Junita shines:

- GPU-first rendering via `wgpu` for fluid 60fps motion and rich visuals.
- Declarative, builder-style API inspired by modern UI toolkits (easy to learn, pleasant to compose).
- Fine-grained reactivity and spring-based motion for natural, interruptible animations.
- Cross-platform: Desktop and Mobile with the same expressive API.

What this book gives you

- A quick ramp from "hello world" to production-ready UI patterns.
- Deep dives on layout, materials, animation, and performance tuning.
- Practical recipes for responsive input, state machines, and platform integration.

What you'll build

By the end of this book you will know how to:

- Compose complex layouts with a flexbox-inspired API.
- Create reusable, animated components with type-safe hooks.
- Apply modern materials (glass, acrylic, blur) and fine-tune rendering.
- Integrate Junita into a native windowed app and ship across platforms.

Quick taste — a tiny Junita app

```rust
use junita_app::prelude::*;

fn main() -> Result<()> {
    WindowedApp::run(WindowConfig::default(), |ctx| {
        div()
            .w(ctx.width).h(ctx.height)
            .bg(Color::rgba(0.06, 0.07, 0.10, 1.0))
            .flex_center()
            .child(
                div()
                    .glass()
                    .rounded(16.0)
                    .p(28.0)
                    .child(text("Welcome to Junita").size(28.0).color(Color::WHITE))
                    .child(text("Fast. Smooth. Delightful.").size(14.0).color(Color::GRAY))
            )
    })
}
```

How this book is organized

- Getting started — installation and your first app
- Core & Layout — elements, layout rules, and styling
- Animation & Motion — springs, timelines, and choreography
- Advanced — performance, custom materials, and platform tips

Ready to go? Jump to the [Getting Started](./getting-started/first-app.md) chapter and let’s make something that moves.
```
            .flex_center()
