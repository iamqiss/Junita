#!/bin/bash
# Script to update metadata for all Junita crates
# Usage: ./scripts/update-crate-metadata.sh

set -e

VERSION="0.1.1"
REPO="https://github.com/project-junita/Junita"

# List of all crates to update
CRATES=(
    "crates/junita_animation"
    "crates/junita_app"
    "crates/junita_cli"
    "crates/junita_cn"
    "crates/junita_core"
    "crates/junita_debugger"
    "crates/junita_gpu"
    "crates/junita_icons"
    "crates/junita_image"
    "crates/junita_layout"
    "crates/junita_macros"
    "crates/junita_paint"
    "crates/junita_platform"
    "crates/junita_recorder"
    "crates/junita_runtime"
    "crates/junita_svg"
    "crates/junita_test_suite"
    "crates/junita_text"
    "crates/junita_theme"
    "extensions/junita_platform_android"
    "extensions/junita_platform_desktop"
    "extensions/junita_platform_ios"
)

echo "Updating metadata for all crates..."

for crate_path in "${CRATES[@]}"; do
    cargo_file="$crate_path/Cargo.toml"
    crate_name=$(basename "$crate_path")

    if [ -f "$cargo_file" ]; then
        echo "Processing $crate_name..."

        # Add repository.workspace if not present
        if ! grep -q "^repository.workspace" "$cargo_file"; then
            sed -i '' '/^license.workspace/a\
repository.workspace = true
' "$cargo_file"
        fi

        # Add rust-version.workspace if not present
        if ! grep -q "^rust-version.workspace" "$cargo_file"; then
            sed -i '' '/^repository.workspace/a\
rust-version.workspace = true
' "$cargo_file"
        fi

        # Add documentation URL if not present
        if ! grep -q "^documentation" "$cargo_file"; then
            sed -i '' "/^repository.workspace/a\\
documentation = \"https://docs.rs/$crate_name\"
" "$cargo_file"
        fi

        # Add version to internal dependencies
        sed -i '' "s/junita_\\([a-z_]*\\) = { path = \"\\([^\"]*\\)\" }/junita_\\1 = { path = \"\\2\", version = \"$VERSION\" }/g" "$cargo_file"
        sed -i '' "s/junita_\\([a-z_]*\\) = { path = \"\\([^\"]*\\)\", optional = true }/junita_\\1 = { path = \"\\2\", version = \"$VERSION\", optional = true }/g" "$cargo_file"
        sed -i '' "s/junita_\\([a-z_]*\\) = { path = \"\\([^\"]*\\)\", default-features = false }/junita_\\1 = { path = \"\\2\", version = \"$VERSION\", default-features = false }/g" "$cargo_file"
        sed -i '' "s/junita_\\([a-z_]*\\) = { path = \"\\([^\"]*\\)\", default-features = false, features = \\[\\([^]]*\\)\\] }/junita_\\1 = { path = \"\\2\", version = \"$VERSION\", default-features = false, features = [\\3] }/g" "$cargo_file"
    else
        echo "Warning: $cargo_file not found"
    fi
done

echo "Done! Metadata updated for all crates."
