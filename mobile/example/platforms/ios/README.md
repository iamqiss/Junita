# example - iOS

## Building

```bash
# Build Rust static library
cargo lipo --release

# Then open Xcode and add the library
```

## Requirements

- Xcode 15+
- Rust iOS targets: `rustup target add aarch64-apple-ios`
- cargo-lipo: `cargo install cargo-lipo`
