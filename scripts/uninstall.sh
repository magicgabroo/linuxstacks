#!/bin/bash
# LinuxStacks - Uninstall script
# Usage: sudo ./scripts/uninstall.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

# Check root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root: sudo $0"
    exit 1
fi

echo ""
echo "╔══════════════════════════════════════╗"
echo "║     LinuxStacks Uninstaller          ║"
echo "╚══════════════════════════════════════╝"
echo ""

# Remove application
if [ -f "/usr/bin/LinuxStacks" ]; then
    rm -f /usr/bin/LinuxStacks
    print_success "Removed /usr/bin/LinuxStacks"
fi

# Remove desktop entry
if [ -f "/usr/share/applications/linuxstacks.desktop" ]; then
    rm -f /usr/share/applications/linuxstacks.desktop
    print_success "Removed desktop entry"
fi

# Remove icons
if [ -d "/usr/share/icons/hicolor/256x256/apps/linuxstacks.png" ]; then
    rm -f /usr/share/icons/hicolor/256x256/apps/linuxstacks.png
    print_success "Removed icon"
fi

# Remove user data (optional - ask user)
read -p "Remove user data in ~/LinuxStacks? [y/N] " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -rf ~/LinuxStacks
    print_success "Removed ~/LinuxStacks"
fi

# Remove config
if [ -f "$HOME/.config/linuxstacks/linuxstacks.toml" ]; then
    rm -f "$HOME/.config/linuxstacks/linuxstacks.toml"
    rmdir "$HOME/.config/linuxstacks" 2>/dev/null || true
    print_success "Removed config"
fi

# Note about dependencies
echo ""
echo -e "${YELLOW}Nota:${NC} Os pacotes do sistema (qemu, libvirt, etc.) não foram removidos."
echo "      Para remover manualmente:"
echo "      sudo apt remove qemu-system-x86 qemu-kvm libvirt-daemon-system"
echo ""
echo -e "${GREEN}Desinstalação concluída!${NC}"
