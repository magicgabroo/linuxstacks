#!/bin/bash
# Build LinuxStacks AppImage

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

echo "Building LinuxStacks AppImage..."

# Check dependencies
command -v cargo >/dev/null 2>&1 || { echo "Rust not installed"; exit 1; }
command -v node >/dev/null 2>&1 || { echo "Node.js not installed"; exit 1; }

# Build frontend
echo "Building frontend..."
npm install
npm run build

# Build Tauri app
echo "Building Tauri app..."
npm run tauri build -- --target x86_64-unknown-linux-gnu

echo "Build complete! AppImage in src-tauri/target/release/bundle/appimage/"
