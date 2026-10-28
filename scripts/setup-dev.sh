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

# Install Node.js/pnpm
if ! command -v pnpm &> /dev/null; then
    echo "Installing pnpm..."
    npm install -g pnpm
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
pnpm install

echo "Development environment ready!"
echo "Run 'pnpm tauri dev' to start development server"