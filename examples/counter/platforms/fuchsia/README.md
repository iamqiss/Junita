# Fuchsia Platform - Counter Example

This directory contains Fuchsia-specific configuration for the Counter example.

## Prerequisites

1. **Fuchsia SDK**: Install via the setup script:
   ```bash
   ./scripts/setup-fuchsia-sdk.sh
   ```

2. **Rust Targets**:
   ```bash
   rustup target add x86_64-unknown-fuchsia
   rustup target add aarch64-unknown-fuchsia
   ```

3. **Fuchsia Emulator (FEMU)**:
   ```bash
   ./scripts/setup-fuchsia-emulator.sh
   ```

## Building

### Cross-compile from Host

```bash
# Build for x86_64 Fuchsia
cargo build --example fuchsia_hello \
    --target x86_64-unknown-fuchsia \
    --features fuchsia \
    --release

# Build for ARM64 Fuchsia
cargo build --example fuchsia_hello \
    --target aarch64-unknown-fuchsia \
    --features fuchsia \
    --release
```

### Create Fuchsia Package

```bash
# Package the binary with the component manifest
# (Requires fuchsia SDK tools)
fx build counter
```

## Running in Emulator

1. **Start FEMU**:
   ```bash
   ffx emu start --headless
   ```

2. **Publish Package**:
   ```bash
   ffx target repository register
   ffx repository add-from-pm counter_package
   ```

3. **Run Component**:
   ```bash
   ffx component run \
       fuchsia-pkg://fuchsia.com/counter#meta/counter.cm
   ```

4. **View Output**:
   - Connect via `ffx target vnc` for display
   - Or use `ffx log` for debug output

## Component Manifest

The component manifest (`meta/counter.cml`) declares:

- **Capabilities Used**:
  - `fuchsia.ui.scenic.Scenic` - Window compositor
  - `fuchsia.vulkan.loader.Loader` - GPU rendering
  - `fuchsia.ui.pointer.*` - Touch/mouse input
  - `fuchsia.ui.input3.Keyboard` - Keyboard input
  - `fuchsia.fonts.Provider` - System fonts

- **Capabilities Exposed**:
  - `fuchsia.ui.app.ViewProvider` - Allows system to embed our view

## Architecture

```
┌─────────────────────────────────────────────┐
│            Fuchsia Component                 │
│  ┌───────────────────────────────────────┐  │
│  │         Blinc Counter App             │  │
│  │  ┌───────────┐    ┌───────────────┐  │  │
│  │  │ UI Tree   │────│ Event Router  │  │  │
│  │  └─────┬─────┘    └───────────────┘  │  │
│  │        │                              │  │
│  │  ┌─────┴─────┐                       │  │
│  │  │ Renderer  │──── wgpu/Vulkan       │  │
│  │  └───────────┘                       │  │
│  └───────────────────────────────────────┘  │
│                     │                        │
│              ┌──────┴──────┐                │
│              │ Scenic View │                │
│              └─────────────┘                │
└─────────────────────────────────────────────┘
        │                    │
   ┌────┴────┐         ┌─────┴─────┐
   │ Display │         │   Input   │
   │ Server  │         │ Pipeline  │
   └─────────┘         └───────────┘
```

## Debugging

### View Logs
```bash
ffx log --filter counter
```

### Inspect Component State
```bash
ffx component show counter
```

### Profile GPU
```bash
ffx trace start --categories vulkan
```

## Resources

- [Fuchsia Component Model](https://fuchsia.dev/fuchsia-src/concepts/components/v2)
- [Scenic Overview](https://fuchsia.dev/fuchsia-src/concepts/graphics/scenic)
- [Vulkan on Fuchsia](https://fuchsia.dev/fuchsia-src/development/graphics/vulkan)
- [Input System](https://fuchsia.dev/fuchsia-src/concepts/ui/input)
