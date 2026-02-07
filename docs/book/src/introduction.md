
# Introduction

<div align="center">

![Junita Window](../../window.svg)

### A GPU-accelerated UI framework that feels alive

</div>

---

## What is Junita?

Junita is a GPU-first, reactive UI framework for Rust — built for developers who want buttery-smooth animations, modern visual effects, and the precision of native rendering without wrestling with low-level graphics APIs.

If you've ever built a UI and thought "this should feel more *alive*", Junita is for you.

## Why Junita exists

Most UI frameworks make you choose:
- **Native performance** but painful low-level APIs
- **Easy declarative syntax** but clunky animations  
- **Beautiful effects** but poor cross-platform support

Junita refuses to compromise. You get all three.

## What makes Junita different

### 🚀 GPU-Accelerated Everything
Every pixel is rendered on the GPU via wgpu. Smooth 60fps animations, complex visual effects like glassmorphism and shadows, resolution-independent text — all hardware-accelerated by default.

### 🎨 Declarative Builder API
Build interfaces with a fluent, composable API inspired by SwiftUI and modern web frameworks. No manual state management, no fighting the framework.

```rust
div()
    .glass()
    .rounded(16.0)
    .p(32.0)
    .child(text("Hello, Junita!"))
```

### ⚡ Spring Physics Animations
Forget tweens and easing curves. Junita uses real spring physics for natural, interruptible motion that responds to user input.

### 🎯 Fine-Grained Reactivity
UI updates automatically when state changes, with surgical precision. Only the components that need to re-render actually do.

### 🌍 True Cross-Platform
Write once, run everywhere. macOS, Windows, Linux, Android — with iOS coming soon. Same expressive API, native performance everywhere.

## What you'll learn

This book will take you from zero to shipping production-ready Junita apps. You'll learn:

- **Core Concepts** — How Junita's reactive system and builder API work
- **Layout Mastery** — Flexbox-powered layouts that adapt to any screen
- **Visual Effects** — Glassmorphism, shadows, blurs, and custom materials
- **Animation** — Spring physics, timelines, and choreographed motion
- **State Management** — Reactive signals, state machines, and data flow
- **Platform Integration** — Native windowing, input handling, and deployment

## Your first taste of Junita

Here's a complete app in just a few lines:

```rust
use junita::prelude::*;

fn main() -> Result<()> {
    App::run(|ctx| {
        div()
            .w(ctx.width).h(ctx.height)
            .bg(Color::from_hex("#1a1a2e"))
            .flex_center()
            .child(
                div()
                    .glass()
                    .rounded(16.0)
                    .p(32.0)
                    .gap(8.0)
                    .child(
                        text("Welcome to Junita")
                            .size(32.0)
                            .weight(700)
                            .color(Color::WHITE)
                    )
                    .child(
                        text("Clear windows into visibility 🪟")
                            .size(16.0)
                            .color(Color::rgb(0.6, 0.6, 0.7))
                    )
            )
    })
}
```

This creates a centered glass card with text, smooth animations, and GPU-accelerated rendering. Run it and feel the difference.

## How this book is organized

**Part 1: Foundations**
- [Getting Started](./getting-started.md) — Installation and your first app
- [Core Concepts](./core-concepts.md) — Understanding Junita's architecture
- [Builder API](./builder-api.md) — Mastering the declarative syntax

**Part 2: Building UIs**
- [Layout System](./layout.md) — Flexbox, sizing, and positioning
- [Styling](./styling.md) — Colors, typography, and visual properties
- [Components](./components.md) — Building reusable UI elements

**Part 3: Motion & Interactivity**
- [Animations](./animations.md) — Spring physics and timelines
- [Gestures](./gestures.md) — Touch, mouse, and keyboard input
- [State Machines](./state-machines.md) — Managing complex UI states

**Part 4: Advanced Topics**
- [Material Effects](./materials.md) — Glass, blur, and custom shaders
- [Performance](./performance.md) — Optimization and debugging
- [Platform Integration](./platform.md) — Native windows and deployment

## Ready to build?

Junita is about making UI development *fun* again. No more fighting CSS quirks, no more janky animations, no more choosing between performance and developer experience.

Let's build interfaces that feel alive.

👉 **[Start with Getting Started →](./getting-started.md)**

---

<div align="center">

🪟 **Clear windows into visibility**

</div>
