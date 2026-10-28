#!/bin/bash
# LinuxStacks - Windows Developer Setup Script
# Run this on Windows to prepare the project for Linux development

set -e

echo "LinuxStacks - Windows Developer Setup"
echo "======================================"

# Check git
if ! command -v git &> $null; then
    echo "[WARN] Git not found. Install from https://git-scm.com/download/win"
else
    echo "[OK] Git found"
fi

# Check Node.js
if ! command -v node &> $null; then
    echo "[WARN] Node.js not found. Install from https://nodejs.org"
else
    echo "[OK] Node.js $(node --version)"
fi

# Check pnpm
if ! command -v pnpm &> $null; then
    echo "[INFO] Installing pnpm..."
    npm install -g pnpm
fi

# Check Rust
if ! command -v cargo &> $null; then
    echo "[WARN] Rust not found. Install from https://rustup.rs"
    echo "  Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
else
    echo "[OK] Rust $(cargo --version)"
fi

# Create git repo if not exists
if [ -d ".git" ]; then
    echo "[OK] Git repo already initialized"
else
    echo "[INFO] Initializing git repo..."
    git init
    git add .
    git commit -m "Initial commit - LinuxStacks project structure"
    echo "[OK] Git repo initialized"
fi

echo ""
echo "Next steps:"
echo "  1. On your Linux machine: git clone https://github.com/magicgabroo/linuxstacks.git"
echo "  2. cd linuxstacks"
echo "  3. ./scripts/setup-dev.sh"
echo "  4. pnpm tauri dev"
