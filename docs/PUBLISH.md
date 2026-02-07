# Cargo Publish Strategy

This document outlines the publishing strategy for Junita crates to crates.io.

## Dependency Graph

```text
Level 0 (no internal deps):
├── junita_macros      (proc-macros)
├── junita_platform    (platform traits)
├── junita_icons       (icon set)
└── junita_core        (reactive system)

Level 1 (depends on Level 0):
├── junita_animation   → junita_core
├── junita_paint       → junita_core
├── junita_recorder    → junita_core
├── junita_svg         → junita_core
├── junita_text        → junita_core
└── junita_platform_desktop → junita_platform

Level 2:
├── junita_theme       → junita_core, junita_animation
└── junita_image       → junita_platform?, junita_text?

Level 3:
└── junita_layout      → junita_core, junita_animation, junita_theme

Level 4:
├── junita_gpu         → junita_core, junita_layout, junita_paint, junita_text
├── junita_cn          → junita_layout, junita_core, junita_animation, junita_theme, junita_macros, junita_icons
├── junita_platform_android → junita_core, junita_animation, junita_platform, junita_gpu?
└── junita_platform_ios     → junita_core, junita_animation, junita_platform, junita_gpu

Level 5:
└── junita_app         → (many deps including platform extensions)

Level 6 (top-level):
├── junita_debugger    → junita_app, junita_layout, junita_theme, junita_cn, junita_icons, junita_recorder
├── junita_cli         → junita_core, junita_animation
├── junita_runtime     → optional deps
└── junita_test_suite  → internal testing
```

## Crate Categories

### Public API (publish to crates.io)

These crates form the public Junita API:

| Crate | Description | Priority |
|-------|-------------|----------|
| `junita_core` | Reactive signals, state machines, types | High |
| `junita_animation` | Spring physics, keyframes, timelines | High |
| `junita_layout` | Layout engine (flexbox, event routing) | High |
| `junita_theme` | Theming system | High |
| `junita_macros` | Procedural macros | High |
| `junita_app` | Application framework | High |
| `junita_gpu` | GPU renderer (wgpu) - for custom renderers | High |
| `junita_paint` | Paint primitives and operations | High |
| `junita_cn` | Component library (buttons, inputs, etc.) | Medium |
| `junita_icons` | Icon set (Lucide icons) | Medium |

### Support Crates (publish as dependencies)

These are needed by public crates and may be useful for advanced users:

| Crate | Description | Needed By |
| ----- | ----------- | --------- |
| `junita_text` | Text shaping/rendering | junita_gpu |
| `junita_svg` | SVG parsing | junita_app |
| `junita_image` | Image loading | junita_app |
| `junita_platform` | Platform abstraction traits | extensions |

### Platform Extensions (publish with target gates)

| Crate | Target | Backend |
|-------|--------|---------|
| `junita_platform_desktop` | macOS/Windows/Linux | wgpu (all backends) |
| `junita_platform_android` | Android | Vulkan |
| `junita_platform_ios` | iOS | Metal |

### Internal Only (do NOT publish)

| Crate | Reason |
|-------|--------|
| `junita_recorder` | Internal debugging tool |
| `junita_debugger` | Internal debugging UI |
| `junita_runtime` | Experimental runtime |
| `junita_test_suite` | Internal test utilities |

## Publish Order

Execute in this exact order (respects dependency graph):

```bash
# Phase 1: Foundation (no internal deps)
cargo publish -p junita_macros
cargo publish -p junita_platform
cargo publish -p junita_icons
cargo publish -p junita_core

# Phase 2: Core systems
cargo publish -p junita_animation
cargo publish -p junita_paint
cargo publish -p junita_svg
cargo publish -p junita_text

# Phase 3: Higher-level systems
cargo publish -p junita_theme
cargo publish -p junita_image
cargo publish -p junita_layout

# Phase 4: GPU and components
cargo publish -p junita_gpu
cargo publish -p junita_cn

# Phase 5: Platform extensions
cargo publish -p junita_platform_desktop
cargo publish -p junita_platform_android
cargo publish -p junita_platform_ios

# Phase 6: Application framework
cargo publish -p junita_app

# Phase 7: CLI (binary)
cargo publish -p junita_cli
```

## Pre-Publish Checklist

### 1. Update Cargo.toml metadata

Each crate needs proper metadata:

```toml
[package]
name = "junita_core"
version = "0.1.1"
edition = "2021"
license = "Apache-2.0"
repository = "https://github.com/project-junita/Junita"
documentation = "https://docs.rs/junita_core"
readme = "README.md"
keywords = ["ui", "gui", "reactive", "framework"]
categories = ["gui", "graphics", "rendering"]
description = "Junita core runtime - reactive signals, state machines, and event dispatch"
```

### 2. Convert path dependencies to version deps

Before publishing, convert path dependencies to version dependencies:

```toml
# Before (development)
junita_core = { path = "../junita_core" }

# After (publish)
junita_core = { version = "0.1.1" }
```

Script to automate this (run before publish):

```bash
# scripts/prepare-publish.sh
for crate in crates/*/Cargo.toml extensions/*/Cargo.toml; do
    sed -i '' 's/path = "\.\.\/junita_\([^"]*\)"/version = "0.1.1"/g' "$crate"
    sed -i '' 's/path = "\.\.\/\.\.\/crates\/junita_\([^"]*\)"/version = "0.1.1"/g' "$crate"
    sed -i '' 's/path = "\.\.\/\.\.\/extensions\/junita_\([^"]*\)"/version = "0.1.1"/g' "$crate"
done
```

### 3. Create README for each crate

Each published crate should have a README.md with:
- Brief description
- Installation instructions
- Basic usage example
- Link to main documentation

### 4. Verify each crate builds standalone

```bash
# Test each crate can build independently
for crate in junita_core junita_animation junita_layout; do
    cargo build -p $crate
done
```

### 5. Run cargo publish --dry-run

```bash
# Verify publish will succeed
cargo publish -p junita_core --dry-run
```

## Version Strategy

### Initial Release (v0.1.1)

- All crates start at `0.1.1`
- Use `0.x.y` to indicate API instability
- Document breaking changes in CHANGELOG.md

### Version Synchronization

Keep all Junita crates at the same version for simplicity:

```toml
# Cargo.toml (workspace)
[workspace.package]
version = "0.1.1"
```

### Semantic Versioning

Once stable (1.0.0+):
- MAJOR: Breaking API changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes only

## Automation

### GitHub Actions Workflow

Create `.github/workflows/publish.yml`:

```yaml
name: Publish to crates.io

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Login to crates.io
        run: cargo login ${{ secrets.CARGO_REGISTRY_TOKEN }}

      - name: Publish crates
        run: |
          # Phase 1
          cargo publish -p junita_macros --no-verify
          cargo publish -p junita_platform --no-verify
          cargo publish -p junita_icons --no-verify
          cargo publish -p junita_core --no-verify
          sleep 30  # Wait for crates.io to index

          # Phase 2
          cargo publish -p junita_animation --no-verify
          cargo publish -p junita_paint --no-verify
          cargo publish -p junita_svg --no-verify
          cargo publish -p junita_text --no-verify
          sleep 30

          # ... continue phases
```

### cargo-release Integration

Consider using [cargo-release](https://github.com/crate-ci/cargo-release) for automated releases:

```bash
cargo install cargo-release
cargo release --workspace 0.1.1
```

## Notes

### Circular Dependencies

There is a dev-dependency cycle between `junita_core` and `junita_animation`:
- `junita_animation` depends on `junita_core` (runtime)
- `junita_core` has dev-dependency on `junita_animation` (tests only)

This does NOT block publishing since dev-dependencies are not considered for the dependency graph.

### Platform-Specific Crates

Platform extensions have target-gated dependencies. When publishing:
- `junita_platform_android` only builds on Android targets
- `junita_platform_ios` only builds on iOS targets
- Use `--no-verify` flag for cross-compilation requirements

### Feature Flags

`junita_app` has feature flags for platforms:
- `default = ["windowed"]` - Desktop with winit
- `android` - Android with Vulkan
- `ios` - iOS with Metal

Consumers select the appropriate feature:

```toml
# Desktop
junita_app = "0.1.1"

# Android
junita_app = { version = "0.1.1", default-features = false, features = ["android"] }

# iOS
junita_app = { version = "0.1.1", default-features = false, features = ["ios"] }
```
