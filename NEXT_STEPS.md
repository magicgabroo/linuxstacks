# LinuxStacks - Pronto para Linux

O projeto foi corrigido e enviado para o GitHub. Agora é só clonar e compilar no Ubuntu.

## Código corrigido

- **Rust**: erros de compilação resolvidos (falta de parênteses, variáveis conflituosas, imports inexistentes)
- **Vue/TypeScript**: scripts duplicados removidos, types corrigidos, eventos Tauri funcionais
- **QEMU**: startup real com tracking de PID, stop via SIGTERM, verificação de processo ativo
- **Instalador**: sem sudo automático — mostra comandos para o usuário executar
- **Assets**: ícones PNG/ICO/SVG criados, favicon adicionado
- **tauri.conf.json**: schema v2 válido, dependências `.deb` adequadas ao Ubuntu 26.04

---

## Instalar no Ubuntu 26.04

```bash
# 1. Clonar
git clone https://github.com/magicgabroo/linuxstacks.git
cd linuxstacks

# 2. Verificar virtualização
lscpu | grep -E 'vmx|svm'    # deve mostrar algo
ls /dev/kvm                   # deve existir

# 3. Instalar dependências do sistema
sudo apt update
sudo apt install -y \
  qemu-system-x86 qemu-kvm \
  libwebkit2gtk-4.1-0 libgtk-3-0 \
  librsvg2-2 patchelf \
  curl git

# 4. Adicionar usuário aos grupos
sudo usermod -aG kvm $USER
sudo usermod -aG libvirt $USER
# FAZER LOGOUT E LOGIN NOVAMENTE PARA OS GRUPOS FUNCIONAREM

# 5. Instalar Rust e pnpm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
npm install -g pnpm

# 6. Instalar dependências do projeto e compilar
pnpm install
pnpm tauri dev          # modo desenvolvimento
pnpm tauri build         # gera o .deb

# 7. Instalar o pacote
sudo apt install ./src-tauri/target/release/bundle/deb/linuxstacks_*.deb
```

---

## Dependências do pacote .deb

```
qemu-system-x86, qemu-kvm, libwebkit2gtk-4.1-0,
libgtk-3-0, librsvg2-2, patchelf
```

Virt-manager e libvirt são **opcionais** — o app usa QEMU diretamente.

---

## Primeiro uso

1. Abra o LinuxStacks
2. Vá em **Configurações → Android** e baixe a imagem Android
3. Ajuste CPU/memória se necessário
4. Clique **Iniciar VM**

---

## Repositório

https://github.com/magicgabroo/linuxstacks
