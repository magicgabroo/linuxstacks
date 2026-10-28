# LinuxStacks - Próximos Passos

## 1. Verificar o código no GitHub
```bash
git clone https://github.com/magicgabroo/linuxstacks.git
cd linuxstacks
```

## 2. Instalar no Ubuntu 26.04 (um comando)
```bash
curl -fsSL https://raw.githubusercontent.com/magicgabroo/linuxstacks/main/scripts/install.sh | bash
```

## 3. Rodar em modo desenvolvimento
```bash
git clone https://github.com/magicgabroo/linuxstacks.git
cd linuxstacks
./scripts/setup-dev.sh
npm run tauri:dev
```

## 4. Build para produção (.deb + AppImage)
```bash
npm run tauri:build
```

## 5. Instalar o .deb
```bash
sudo apt install ./src-tauri/target/release/bundle/deb/linuxstacks_*.deb
```

---

## Dependências do sistema
- qemu-system-x86 (≥ 6.0)
- qemu-kvm
- libwebkit2gtk-4.1-0 (≥ 2.34)
- libgtk-3-0
- librsvg2-2
- patchelf
- libayatana-appindicator3-1

## Requisitos de hardware
- CPU com virtualização (VT-x/AMD-V)
- /dev/kvm acessível
- Mínimo 4GB RAM (recomendado 8GB)
- Espaço em disco: 2GB+ para imagem Android

## Desinstalação
```bash
curl -fsSL https://raw.githubusercontent.com/magicgabroo/linuxstacks/main/scripts/uninstall.sh | bash
```
