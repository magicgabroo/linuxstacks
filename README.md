# LinuxStacks 🐧📱

> **LinuxStacks** - Um emulador Android nativo para Linux, inspirado no BlueStacks mas feito para Linux.
> Interface moderna, zero telemetria, zero bloatware, 100% open source.

[![Build Status](https://github.com/magicgabroo/linuxstacks/workflows/Build/badge.svg)](https://github.com/magicgabroo/linuxstacks/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)

---

## ✨ Features

| Feature | Status |
|---------|--------|
| 🚀 Android via KVM/QEMU (nativo, rápido) | ✅ |
| 🎨 UI Tauri + Vue 3 (leve, nativa) | ✅ |
| 📦 AppImage / .deb | ✅ |
| 🔒 Zero telemetria, zero anúncios | ✅ |
| 📁 Compartilhamento de arquivos host↔guest | ✅ |
| 🎮 Mapeamento de teclado | ✅ |
| 📱 ADB integration (porta 5555) | ✅ |
| 🔄 Assistente de instalação | ✅ |
| ⚙️ Configuração completa de VM | ✅ |

---

## 🚀 Instalação Rápida (Ubuntu 26.04)

```bash
# Opção 1: Script único (recomendado)
curl -fsSL https://raw.githubusercontent.com/magicgabroo/linuxstacks/main/scripts/install.sh | bash

# Opção 2: Baixar .deb do release
wget https://github.com/magicgabroo/linuxstacks/releases/latest/download/LinuxStacks_*.deb
sudo apt install ./LinuxStacks_*.deb

# Opção 3: AppImage
wget https://github.com/magicgabroo/linuxstacks/releases/latest/download/LinuxStacks-x86_64.AppImage
chmod +x LinuxStacks-x86_64.AppImage
./LinuxStacks-x86_64.AppImage
```

---

## 🛠️ Desenvolvimento

### Pré-requisitos

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y \
    git curl wget \
    libwebkit2gtk-4.1-dev \
    libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
    librsvg2-dev patchelf \
    qemu-system-x86 qemu-kvm libvirt-daemon-system \
    nodejs npm

# Arch/Manjaro
sudo pacman -S --needed base-devel git curl wget \
    webkit2gtk-4.1 openssl gtk3 libayatana-appindicator \
    librsvg patchelf qemu-system-x86 qemu-kvm libvirt \
    nodejs npm

# Fedora
sudo dnf install @development-tools git curl wget \
    webkit2gtk4.1 openssl-devel gtk3-devel \
    libayatana-appindicator-gtk3-devel librsvg2-devel \
    patchelf qemu-system-x86 qemu-kvm libvirt \
    nodejs npm
```

### Build Local

```bash
# Clone
git clone https://github.com/magicgabroo/linuxstacks.git
cd linuxstacks

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install dependencies
npm install

# Dev mode (hot reload)
npm run tauri:dev

# Build production
npm run tauri:build
```

---

## 📁 Estrutura do Projeto

```
linuxstacks/
├── .github/workflows/       # CI/CD (AppImage, .deb)
├── src-tauri/               # Backend Rust
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── vm_manager.rs    # QEMU/KVM control
│   │   ├── config.rs        # Config TOML
│   │   ├── installer.rs     # Instalação automatizada
│   │   └── error.rs         # Error handling
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── src/                     # Frontend Vue 3
│   ├── components/          # VmWindow, Sidebar, Toolbar, Settings
│   ├── stores/              # Pinia state management
│   ├── styles/              # CSS/Tailwind
│   └── types/
├── scripts/
│   ├── install.sh           # Instalação one-liner
│   ├── uninstall.sh         # Desinstalação
│   ├── build-appimage.sh
│   └── download-android.sh
├── linuxstacks.toml.example # Config exemplo
└── Cargo.toml (workspace)
```

---

## ⚙️ Configuração

### linuxstacks.toml

```toml
[vm]
memory_mb = 4096
cpus = 4
gpu = "virtio-gpu"
network = "user-mode"
disk_path = "~/LinuxStacks/android.qcow2"

[android]
image_url = ""  # URL da imagem Android x86_64
shared_folder = "~/LinuxStacks/shared"

[ui]
theme = "auto"      # auto/light/dark
language = "pt-BR"
show_toolbar = true

[features]
f_droid = true
aurora_store = true
adb_integration = true
clipboard_sync = true
file_share = true
```

---

## 📦 Imagem Android

Para usar o LinuxStacks, você precisa de uma imagem Android x86_64:

```bash
# Opção 1: Usar o script de download
./scripts/download-android.sh

# Opção 2: Baixar manualmente
# https://developer.android.com/tools/releases/images
# Ou usar o Android SDK:
# sdkmanager "system-images;android-34;google_apis;x86_64"
```

Extraia e configure a URL no menu Configurações > Android.

---

## 📦 Distribuição

### .deb (Ubuntu/Debian)
```bash
# O build do GitHub gera automaticamente
sudo apt install ./LinuxStacks_*.deb
```

### AppImage (Universal)
```bash
# Build local
./scripts/build-appimage.sh
# Output: src-tauri/target/release/bundle/appimage/LinuxStacks*.AppImage
```

---

## 🗺️ Roadmap

- [x] Core: KVM/QEMU + Tauri backend
- [x] UI: Vue 3 + Tauri frontend
- [x] Installer: Script universal
- [x] Assistente de instalação
- [ ] F-Droid / Aurora Store UI
- [ ] ADB device manager
- [ ] Snapshot/clone VMs
- [ ] Gamepad/keymapper
- [ ] Multi-VM support
- [ ] Wayland native support

---

## 🤝 Contribuindo

```bash
# 1. Fork
# 2. Branch: git checkout -b feature/nova-feature
# 3. Commit: git commit -m "feat: nova feature"
# 4. Push: git push origin feature/nova-feature
# 5. PR
```

---

## 📄 Licença

MIT License - veja [LICENSE](LICENSE)

---

## 🙏 Agradecimentos

- [Tauri](https://tauri.app/) - Framework desktop
- [QEMU/KVM](https://www.qemu.org/) - Virtualização
- [Vue.js](https://vuejs.org/) - Frontend
- [Rust](https://www.rust-lang.org/) - Backend

---

**Feito com ❤️ para a comunidade Linux**
