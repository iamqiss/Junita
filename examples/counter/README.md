# counter

A Junita UI application.

## Development

```bash
junita dev
```

## Build

```bash
# Desktop (current platform)
junita build --release

# Mobile
junita build --target android --release
junita build --target ios --release
```

## Project Structure

```
counter/
├── .junitaproj           # Project configuration
├── src/
│   └── main.junita       # Application entry point
├── assets/              # Static assets (images, fonts, etc.)
├── plugins/             # Local plugins
└── platforms/           # Platform-specific code
    ├── android/         # Android project files
    ├── ios/             # iOS/Xcode project files
    ├── macos/           # macOS app bundle config
    ├── windows/         # Windows executable config
    └── linux/           # Linux desktop config
```

## Configuration

Edit `.junitaproj` to configure:
- Project metadata (name, version, description)
- Platform-specific settings (package IDs, SDK versions)
- Dependencies (plugins, external packages)
