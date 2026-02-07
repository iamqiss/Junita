# Junita iOS Swift Integration

This directory contains Swift files for integrating Junita into iOS applications.

## Files

- **Junita-Bridging-Header.h** - C header declaring the Rust FFI functions
- **JunitaViewController.swift** - Example UIViewController with CADisplayLink integration

## Setup

### 1. Build the Rust library

```bash
# Build for iOS simulator (arm64)
cargo build --release --target aarch64-apple-ios-sim -p junita_app --features ios

# Build for iOS device (arm64)
cargo build --release --target aarch64-apple-ios -p junita_app --features ios
```

### 2. Add to Xcode project

1. Add the static library (`libjunita_app.a`) to your Xcode project
2. Add the bridging header path to your build settings:
   - `Objective-C Bridging Header: path/to/Junita-Bridging-Header.h`
3. Link required frameworks:
   - `Metal.framework`
   - `MetalKit.framework`
   - `QuartzCore.framework`

### 3. Use in your app

```swift
import UIKit

class MyJunitaViewController: JunitaViewController {

    override func renderFrame() {
        // This is called by CADisplayLink when rendering is needed
        // Build your UI and render with Metal here

        // Example:
        // 1. Access junitaContext to call build_ui
        // 2. Get the render tree
        // 3. Render to the Metal layer
    }
}
```

## Touch Phase Values

The `junita_handle_touch` function accepts a `phase` parameter:

| Value | Phase | Description |
|-------|-------|-------------|
| 0 | Began | Touch started |
| 1 | Moved | Touch position changed |
| 2 | Ended | Touch lifted |
| 3 | Cancelled | Touch cancelled by system |

## Thread Safety

- All Junita FFI functions must be called from the main thread
- CADisplayLink callbacks run on the main thread by default
- The render context is not thread-safe

## Memory Management

- Call `junita_create_context` once during initialization
- Call `junita_destroy_context` when done (e.g., in `deinit`)
- The context pointer is owned by Swift after creation
