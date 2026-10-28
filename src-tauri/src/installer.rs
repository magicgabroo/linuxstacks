use std::process::Command;
use std::path::PathBuf;
use tokio::process::Command as TokioCommand;
use std::fs;
use crate::config::{AppConfig, VmConfig};
use crate::error::{LinuxStacksError, Result};

pub struct Installer {
    config: AppConfig,
}

impl Installer {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    
    pub fn check_prerequisites() -> Result<PrerequisiteCheck> {
        let mut check = PrerequisiteCheck::default();
        
        // Check KVM
        check.kvm = Self::check_kvm().is_ok();
        
        // Check QEMU
        check.qemu = Self::check_qemu().is_ok();
        
        // Check libvirt
        check.libvirt = Self::check_libvirt().is_ok();
        
        // Check kernel modules
        check.kvm_modules = Self::check_kvm_modules().is_ok();
        
        // Check user groups
        check.user_in_kvm_group = Self::check_user_in_kvm_group();
        
        check.all_ok = check.kvm && check.qemu && check.libvirt && check.kvm_modules && check.user_in_kvm_group;
        
        Ok(check)
    }
    
    fn check_kvm() -> Result<()> {
        let kvm_path = std::path::Path::new("/dev/kvm");
        if !kvm_path.exists() {
            return Err(LinuxStacksError::KvmUnavailable("/dev/kvm not found".into()));
        }
        
        // Check permissions
        let metadata = std::fs::metadata("/dev/kvm")?;
        use std::os::unix::fs::MetadataExt;
        let mode = metadata.mode();
        if mode & 0o666 == 0 {
            return Err(LinuxStacksError::Permission("/dev/kvm not readable/writable".into()));
        }
        
        Ok(())
    }
    
    fn check_qemu() -> Result<()> {
        let output = Command::new("qemu-system-x86_64")
            .arg("--version")
            .output()?;
        
        if !output.status.success() {
            return Err(LinuxStacksError::Vm("qemu-system-x86_64 not working".into()));
        }
        Ok(())
    }
    
    fn check_libvirt() -> Result<()> {
        let output = Command::new("virsh")
            .arg("--version")
            .output()?;
        
        if !output.status.success() {
            return Err(LinuxStacksError::Vm("libvirt not available".into()));
        }
        Ok(())
    }
    
    fn check_kvm_modules() -> Result<()> {
        let kvm_intel = std::path::Path::new("/sys/module/kvm_intel");
        let kvm_amd = std::path::Path::new("/sys/module/kvm_amd");
        
        if !kvm_intel.exists() && !kvm_amd.exists() {
            return Err(LinuxStacksError::KvmUnavailable("KVM kernel modules not loaded".into()));
        }
        Ok(())
    }
    
    fn check_user_in_kvm_group() -> bool {
        if let Ok(groups) = Command::new("groups").output() {
            let groups_str = String::from_utf8_lossy(&groups.stdout);
            groups_str.contains("kvm")
        } else {
            false
        }
    }
    
    pub async fn install_dependencies(&self) -> Result<()> {
        // Detect distro and install appropriate packages
        let distro = self.detect_distro()?;
        
        match distro.as_str() {
            "ubuntu" | "debian" => self.install_debian().await,
            "arch" | "manjaro" => self.install_arch().await,
            "fedora" => self.install_fedora().await,
            "opensuse" => self.install_opensuse().await,
            _ => Err(LinuxStacksError::Install(format!("Unsupported distro: {}", distro))),
        }
    }
    
    fn detect_distro(&self) -> Result<String> {
        let content = fs::read_to_string("/etc/os-release")?;
        for line in content.lines() {
            if line.starts_with("ID=") {
                return Ok(line.trim_start_matches("ID=").trim_matches('"').to_string());
            }
        }
        Ok("unknown".to_string())
    }
    
    async fn install_debian(&self) -> Result<()> {
        let packages = vec![
            "qemu-system-x86",
            "qemu-kvm",
            "libvirt-daemon-system",
            "virt-manager",
            "libvirt-clients",
            "bridge-utils",
            "libwebkit2gtk-4.1-0",
            "libgtk-3-0",
            "libayatana-appindicator3-1",
            "librsvg2-2",
            "patchelf",
        ];
        
        self.run_package_install("apt", &packages).await
    }
    
    async fn install_arch(&self) -> Result<()> {
        let packages = vec![
            "qemu-system-x86",
            "qemu-kvm",
            "libvirt",
            "virt-manager",
            "webkit2gtk-4.1",
            "gtk3",
            "libayatana-appindicator",
            "librsvg",
            "patchelf",
        ];
        
        self.run_package_install("pacman", &packages).await
    }
    
    async fn install_fedora(&self) -> Result<()> {
        let packages = vec![
            "qemu-system-x86",
            "qemu-kvm",
            "libvirt-daemon",
            "virt-manager",
            "webkit2gtk4.1",
            "gtk3",
            "libayatana-appindicator-gtk3",
            "librsvg2",
            "patchelf",
        ];
        
        self.run_package_install("dnf", &packages).await
    }
    
    async fn install_opensuse(&self) -> Result<()> {
        let packages = vec![
            "qemu-system-x86",
            "qemu-kvm",
            "libvirt-daemon",
            "virt-manager",
            "webkit2gtk-4.1",
            "gtk3",
            "libayatana-appindicator-gtk3",
            "librsvg2",
            "patchelf",
        ];
        
        self.run_package_install("zypper", &packages).await
    }
    
    async fn run_package_install(&self, manager: &str, packages: &[&str]) -> Result<()> {
        let args = match manager {
            "apt" => vec!["install", "-y"],
            "pacman" => vec!["-S", "--needed", "--noconfirm"],
            "dnf" => vec!["install", "-y"],
            "zypper" => vec!["install", "-y"],
            _ => return Err(LinuxStacksError::Install("Unknown package manager".into())),
        };
        
        let mut cmd = TokioCommand::new("sudo");
        cmd.arg(manager).args(args).args(packages);
        
        let output = cmd.output().await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(LinuxStacksError::Install(format!("Package install failed: {}", stderr)));
        }
        
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct PrerequisiteCheck {
    pub kvm: bool,
    pub qemu: bool,
    pub libvirt: bool,
    pub kvm_modules: bool,
    pub user_in_kvm_group: bool,
    pub all_ok: bool,
}

impl PrerequisiteCheck {
    pub fn summary(&self) -> String {
        format!(
            "KVM: {} | QEMU: {} | Libvirt: {} | Modules: {} | User in kvm: {}",
            if self.kvm { "✅" } else { "❌" },
            if self.qemu { "✅" } else { "❌" },
            if self.libvirt { "✅" } else { "❌" },
            if self.kvm_modules { "✅" } else { "❌" },
            if self.user_in_kvm_group { "✅" } else { "❌" },
        )
    }
}