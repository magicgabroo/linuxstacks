# LinuxStacks - Próximos Passos

## No GitHub (neste computador Windows)

### Opção 1: Via GitHub CLI (recomendado)
```powershell
# Instale o GitHub CLI se não tiver:
# https://cli.github.com/

gh auth login          # Faça login
gh repo create magicgabroo/linuxstacks --private --source=. --push
```

### Opção 2: Via interface web
1. Acesse https://github.com/new
2. Crie repo `linuxstacks` (público ou privado)
3. Faça push manual quando tiver Git instalado:
```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/magicgabroo/linuxstacks.git
git branch -M main
git push -u origin main
```

---

## No Linux (quando estiver pronto)

```bash
# 1. Clone
git clone https://github.com/magicgabroo/linuxstacks.git
cd linuxstacks

# 2. Verifique KVM
lscpu | grep -E 'vmx|svm'
ls /dev/kvm

# 3. Setup dev environment
./scripts/setup-dev.sh

# 4. Desenvolvimento
pnpm tauri dev

# 5. Build produção
pnpm tauri build
```

---

## Instalação Producao (Linux)

```bash
# Ap os primeiros releases:
curl -fsSL https://raw.githubusercontent.com/magicgabroo/linuxstacks/main/scripts/install.sh | bash
```

---

## Observações

- **Git não instalado neste Windows**: use PowerShell Git Bash ou WSL para o primeiro push
- **Icons**: PNGs são placeholders 1x1px — gere ícones reais e substitua em `src-tauri/icons/`
- **Lockfile**: `Cargo.lock` será gerado automaticamente no primeiro build no Linux
- **CI/CD**: O workflow no GitHub Actions criará releases automaticamente ao dar tag `v*`
