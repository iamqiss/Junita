# junita_cli

> **Part of the [Junita UI Framework](https://project-junita.github.io/Junita)**
>
> This crate is a component of Junita, a GPU-accelerated UI framework for Rust.
> For full documentation and guides, visit the [Junita documentation](https://project-junita.github.io/Junita).

Command-line interface for the Junita UI framework.

## Overview

`junita_cli` provides commands for building, running, and developing Junita applications with features like hot-reload.

## Installation

```bash
cargo install junita_cli
```

Or build from source:

```bash
cargo build -p junita_cli --release
```

## Commands

### Build

Compile your Junita application:

```bash
# Build for current platform
junita build

# Build for specific platform
junita build --platform macos
junita build --platform windows
junita build --platform linux
junita build --platform android
junita build --platform ios

# Release build
junita build --release

# Specify target directory
junita build --target-dir ./build
```

### Dev

Run in development mode with hot-reload:

```bash
# Start dev server
junita dev

# Specify port
junita dev --port 3000

# Watch specific directories
junita dev --watch src --watch assets

# Disable hot-reload
junita dev --no-hot-reload
```

### Doctor

Check your development environment:

```bash
junita doctor
```

Output:
```
Checking Junita development environment...

✓ Rust toolchain: 1.75.0
✓ Cargo: 1.75.0
✓ wgpu supported: Yes
✓ Platform SDK: macOS 14.0
✓ Android SDK: Not found (optional)
✓ iOS SDK: Xcode 15.0

Environment is ready for Junita development!
```

## Configuration

Create a `Junita.toml` in your project root:

```toml
[package]
name = "my-app"
version = "0.1.0"

[build]
target-dir = "target"
assets = ["assets"]

[dev]
port = 3000
hot-reload = true
watch = ["src", "assets"]

[platforms.macos]
bundle-id = "com.example.myapp"
min-version = "11.0"

[platforms.ios]
bundle-id = "com.example.myapp"
min-version = "14.0"

[platforms.android]
package = "com.example.myapp"
min-sdk = 24
target-sdk = 34
```

## Project Structure

```
my-app/
├── Junita.toml          # Project configuration
├── Cargo.toml          # Rust dependencies
├── src/
│   └── main.rs         # Application entry point
└── assets/             # Images, fonts, etc.
```

## Platform Requirements

### macOS
- Xcode Command Line Tools
- macOS 11.0+

### Windows
- Visual Studio Build Tools
- Windows 10+

### Linux
- GCC or Clang
- X11 or Wayland development packages

### Android
- Android SDK
- Android NDK
- Java JDK

### iOS
- Xcode
- iOS Simulator or device

## License

MIT OR Apache-2.0
