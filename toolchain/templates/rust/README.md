# {{project_name}}

A Junita UI application with cross-platform support for desktop, Android, and iOS.

## Quick Start

### Desktop

```bash
cargo run --features desktop
```

### Android

```bash
# Build Rust library
cargo ndk -t arm64-v8a build --lib

# Build and install APK
cd platforms/android
./gradlew installDebug
```

### iOS

```bash
# Build Rust library
cargo lipo --release

# Open Xcode project and run
open platforms/ios/JunitaApp.xcodeproj
```

## Project Structure

```
{{project_name}}/
├── Cargo.toml           # Rust project configuration
├── junita.toml           # Junita toolchain configuration
├── src/
│   └── main.rs          # Application code
└── platforms/
    ├── android/         # Android Gradle project
    └── ios/             # iOS Xcode project files
```

## Native Bridge

Call platform-native functions from Rust:

```rust
use junita_core::native_bridge::native_call;

// Get device info
let battery: String = native_call("device", "get_battery_level", ()).unwrap();
let model: String = native_call("device", "get_model", ()).unwrap();

// Haptic feedback
let _ = native_call::<(), (i32,)>("haptics", "vibrate", (100,));

// Clipboard
let _ = native_call::<(), (String,)>("clipboard", "copy", ("Hello!".to_string(),));
```

## Documentation

- [Android Setup](platforms/android/README.md)
- [iOS Setup](platforms/ios/README.md)
- [Junita Documentation](https://github.com/anthropics/junita)
