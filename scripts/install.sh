#!/bin/bash
# LinuxStacks - One-line installer
# Usage: curl -fsSL https://raw.githubusercontent.com/magicgabroo/linuxstacks/main/scripts/install.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

REPO="magicgabroo/linuxstacks"
INSTALL_DIR="$HOME/.local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_root() {
    if [ "$EUID" -eq 0 ]; then
        print_error "Não execute como root! Execute como usuário normal."
        exit 1
    fi
}

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
        VERSION=$VERSION_ID
    else
        print_error "Não foi possível detectar a distribuição"
        exit 1
    fi
}

check_kvm() {
    if [ ! -e /dev/kvm ]; then
        print_error "/dev/kvm não encontrado. Ative a virtualização na BIOS."
        exit 1
    fi
    
    if [ ! -r /dev/kvm ] || [ ! -w /dev/kvm ]; then
        print_warning "Usuário não tem permissão em /dev/kvm. Adicionando ao grupo kvm..."
        sudo usermod -aG kvm $USER
        print_warning "Faça logout/login para aplicar as mudanças de grupo"
    fi
}

install_dependencies() {
    print_step "Instalando dependências para $DISTRO..."
    
    case $DISTRO in
        ubuntu|debian|linuxmint|pop|elementary)
            sudo apt update
            sudo apt install -y \
                qemu-system-x86 qemu-kvm \
                libvirt-daemon-system libvirt-clients \
                virt-manager bridge-utils \
                libwebkit2gtk-4.1-0 libgtk-3-0 \
                libayatana-appindicator3-1 librsvg2-2 \
                patchelf curl wget
            ;;
        arch|manjaro|endeavouros|garuda)
            sudo pacman -S --needed --noconfirm \
                qemu-system-x86 qemu-kvm \
                libvirt virt-manager \
                webkit2gtk-4.1 gtk3 \
                libayatana-appindicator librsvg \
                patchelf curl wget
            # Enable libvirt
            sudo systemctl enable --now libvirtd
            ;;
        fedora|rhel|centos|rocky|almalinux)
            sudo dnf install -y \
                qemu-system-x86 qemu-kvm \
                libvirt-daemon virt-manager \
                webkit2gtk4.1 gtk3 \
                libayatana-appindicator-gtk3 librsvg2 \
                patchelf curl wget
            sudo systemctl enable --now libvirtd
            ;;
        opensuse*|sles)
            sudo zypper install -y \
                qemu-system-x86 qemu-kvm \
                libvirt-daemon virt-manager \
                webkit2gtk-4.1 gtk3 \
                libayatana-appindicator-gtk3 librsvg2 \
                patchelf curl wget
            sudo systemctl enable --now libvirtd
            ;;
        *)
            print_error "Distribuição não suportada: $DISTRO"
            print_warning "Instale manualmente: qemu-system-x86, qemu-kvm, libvirt, virt-manager, webkit2gtk, gtk3, patchelf"
            ;;
    esac
    
    # Add user to libvirt group
    sudo usermod -aG libvirt $USER
    
    # Enable and start libvirtd
    sudo systemctl enable --now libvirtd 2>/dev/null || true
}

download_appimage() {
    print_step "Baixando LinuxStacks AppImage..."
    
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$DESKTOP_DIR"
    mkdir -p "$ICON_DIR"
    
    # Get latest release
    LATEST_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*AppImage" | cut -d '"' -f 4 | head -1)
    
    if [ -z "$LATEST_URL" ]; then
        print_warning "Não foi possível encontrar release. Tentando build local..."
        return 1
    fi
    
    print_step "Baixando de $LATEST_URL..."
    wget -q --show-progress -O "$INSTALL_DIR/LinuxStacks.AppImage" "$LATEST_URL"
    chmod +x "$INSTALL_DIR/LinuxStacks.AppImage"
    
    print_success "AppImage baixado em $INSTALL_DIR/LinuxStacks.AppImage"
}

create_desktop_entry() {
    print_step "Criando entrada no menu de aplicativos..."
    
    cat > "$DESKTOP_DIR/linuxstacks.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=LinuxStacks
Comment=Android emulator for Linux
Exec=$INSTALL_DIR/LinuxStacks.AppImage
Icon=linuxstacks
Terminal=false
Type=Application
Categories=Development;Emulator;
StartupNotify=true
Keywords=android;emulator;kvm;qemu;
EOF
    
    # Download icon
    ICON_URL="https://raw.githubusercontent.com/$REPO/main/src-tauri/icons/256x256.png"
    wget -q -O "$ICON_DIR/linuxstacks.png" "$ICON_URL" 2>/dev/null || true
    
    print_success "Entrada no menu criada"
}

create_directories() {
    print_step "Criando diretórios..."
    mkdir -p "$HOME/LinuxStacks"/{shared,images,snapshots}
    print_success "Diretórios criados em ~/LinuxStacks"
}

main() {
    echo -e "${GREEN}"
    echo "╔══════════════════════════════════════╗"
    echo "║       LinuxStacks Installer          ║"
    echo "║   Android Emulator for Linux         ║"
    echo "╚══════════════════════════════════════╝"
    echo -e "${NC}"
    
    check_root
    detect_distro
    
    print_step "Detectado: $PRETTY_NAME ($DISTRO $VERSION)"
    
    # Check KVM
    print_step "Verificando virtualização..."
    check_kvm
    print_success "KVM disponível"
    
    # Install dependencies
    install_dependencies
    
    # Create directories
    create_directories
    
    # Try to download AppImage
    if download_appimage; then
        create_desktop_entry
    else
        print_warning "AppImage não disponível. Build local necessário."
        print_step "Para build local:"
        echo "  git clone https://github.com/$REPO.git"
        echo "  cd linuxstacks"
        echo "  npm install && npm run tauri:build"
    fi
    
    echo -e "\n${GREEN}═══════════════════════════════════════${NC}"
    echo -e "${GREEN}Instalação concluída!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════${NC}"
    echo ""
    echo "Próximos passos:"
    echo "  1. Faça logout/login (para aplicar grupos kvm/libvirt)"
    echo "  2. Execute 'LinuxStacks' no menu de aplicativos"
    echo "  3. Ou execute: $INSTALL_DIR/LinuxStacks.AppImage"
    echo ""
    echo "Configuração salva em: ~/.config/linuxstacks/linuxstacks.toml"
    echo "Dados da VM em: ~/LinuxStacks"
}

main "$@"