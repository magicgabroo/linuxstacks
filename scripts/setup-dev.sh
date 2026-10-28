#!/bin/bash
# Setup development environment for LinuxStacks

set -e

echo "Setting up LinuxStacks development environment..."

# Install Rust
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install Node.js/npm check
if ! command -v node &> /dev/null; then
    echo "[WARN] Node.js not found. Install from https://nodejs.org"
else
    echo "[OK] Node.js $(node --version)"
fi

# Install system dependencies (Ubuntu/Debian)
if command -v apt &> /dev/null; then
    echo "Installing system dependencies..."
    sudo apt update
    sudo apt install -y \
        libwebkit2gtk-4.1-dev \
        libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
        librsvg2-dev patchelf \
        qemu-system-x86 qemu-kvm libvirt-daemon-system \
        curl wget git
fi

# Install project dependencies
echo "Installing project dependencies..."
npm install

echo ""
echo "Development environment ready!"
echo ""
echo "Commands:"
echo "  npm run tauri:dev     # Start dev server with hot reload"
echo "  npm run tauri:build   # Build production app"
echo "  npm run build         # Build frontend only"
