#!/bin/bash
# Build LinuxStacks AppImage

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

echo "Building LinuxStacks AppImage..."

# Check dependencies
command -v cargo >/dev/null 2>&1 || { echo "Rust not installed"; exit 1; }
command -v pnpm >/dev/null 2>&1 || { echo "pnpm not installed"; exit 1; }

# Build frontend
echo "Building frontend..."
pnpm install
pnpm build

# Build Tauri app
echo "Building Tauri app..."
cd src-tauri
cargo tauri build --target x86_64-unknown-linux-gnu

echo "Build complete! AppImage in src-tauri/target/release/bundle/appimage/"