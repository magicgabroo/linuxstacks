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
| 📦 AppImage / Flatpak / Snap | ✅ |
| 🔒 Zero telemetria, zero anúncios | ✅ |
| 🛒 F-Droid + Aurora Store integrado | ✅ |
| 📁 Compartilhamento de arquivos host↔guest | ✅ |
| 🎮 Mapeamento de teclado/gamepad | ✅ |
| 📱 ADB integration | ✅ |
| 🔄 Snapshot/backup de VMs | 🚧 |

---

## 🚀 Instalação Rápida (Linux)

```bash
# Opção 1: Script único (recomendado)
curl -fsSL https://raw.githubusercontent.com/magicgabroo/linuxstacks/main/scripts/install.sh | bash

# Opção 2: Baixar AppImage
wget https://github.com/magicgabroo/linuxstacks/releases/latest/download/LinuxStacks-x86_64.AppImage
chmod +x LinuxStacks-x86_64.AppImage
./LinuxStacks-x86_64.AppImage

# Opção 3: Flatpak
flatpak install flathub org.linuxstacks.LinuxStacks
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
    python3-pip nodejs npm

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

# Install pnpm
npm install -g pnpm

# Install dependencies
pnpm install

# Dev mode (hot reload)
pnpm tauri dev

# Build production
pnpm tauri build
```

---

## 📁 Estrutura do Projeto

```
linuxstacks/
├── .github/workflows/       # CI/CD (AppImage, Flatpak, Snap)
├── src-tauri/               # Backend Rust
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── vm_manager.rs    # QEMU/KVM control
│   │   ├── config.rs        # Config TOML
│   │   ├── installer.rs     # Instalação automatizada
│   │   ├── ipc.rs           # Comunicação frontend↔backend
│   │   └── error.rs         # Error handling
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── src/                     # Frontend Vue 3
│   ├── components/          # VmWindow, Sidebar, Toolbar, Settings
│   ├── stores/              # Pinia state management
│   ├── styles/              # CSS/Tailwind
│   └── utils/
├── scripts/
│   ├── install.sh           # Instalação one-liner
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
image_url = "https://dl.google.com/android/repository/android-x86_64.img"
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

## 📦 Distribuição

### AppImage (Universal)
```bash
# Build
./scripts/build-appimage.sh

# Output: LinuxStacks-x86_64.AppImage
```

### Flatpak
```bash
# Build
flatpak-builder build-dir org.linuxstacks.LinuxStacks.yml --force-clean

# Install
flatpak install --user build-dir
```

---

## 🗺️ Roadmap

- [x] Core: KVM/QEMU + Tauri backend
- [x] UI: Vue 3 + Tauri frontend
- [x] Installer: Script universal
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
- [android-x86](https://www.android-x86.org/) - Android x86
- [Vue.js](https://vuejs.org/) - Frontend
- [Rust](https://www.rust-lang.org/) - Backend

---

**Feito com ❤️ para a comunidade Linux**