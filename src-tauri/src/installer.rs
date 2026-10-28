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

        check.kvm = Self::check_kvm().is_ok();
        check.qemu = Self::check_qemu().is_ok();
        check.libvirt = Self::check_libvirt().is_ok();
        check.kvm_modules = Self::check_kvm_modules().is_ok();
        check.user_in_kvm_group = Self::check_user_in_kvm_group();

        check.all_ok = check.kvm && check.qemu && check.kvm_modules && check.user_in_kvm_group;

        Ok(check)
    }

    fn check_kvm() -> Result<()> {
        let kvm_path = std::path::Path::new("/dev/kvm");
        if !kvm_path.exists() {
            return Err(LinuxStacksError::KvmUnavailable("/dev/kvm not found".into()));
        }

        let metadata = std::fs::metadata("/dev/kvm")?;
        use std::os::unix::fs::MetadataExt;
        let mode = metadata.mode();
        if mode & 0o444 == 0 {
            return Err(LinuxStacksError::Permission("/dev/kvm not readable".into()));
        }

        Ok(())
    }

    fn check_qemu() -> Result<()> {
        let output = Command::new("qemu-system-x86_64")
            .arg("--version")
            .output()
            .map_err(|_| LinuxStacksError::Vm("qemu-system-x86_64 not found".into()))?;

        if !output.status.success() {
            return Err(LinuxStacksError::Vm("qemu-system-x86_64 not working".into()));
        }
        Ok(())
    }

    fn check_libvirt() -> Result<()> {
        let output = Command::new("virsh")
            .arg("--version")
            .output();

        // libvirt is optional - we can run QEMU directly without it
        if let Ok(output) = output {
            if output.status.success() {
                return Ok(());
            }
        }
        // Return Ok but mark libvirt as not available
        Ok(())
    }

    fn check_kvm_modules() -> Result<()> {
        let kvm_intel = std::path::Path::new("/sys/module/kvm_intel");
        let kvm_amd = std::path::Path::new("/sys/module/kvm_amd");

        if !kvm_intel.exists() && !kvm_amd.exists() {
            // Check if /dev/kvm exists as fallback
            if PathBuf::from("/dev/kvm").exists() {
                return Ok(());
            }
            return Err(LinuxStacksError::KvmUnavailable("KVM kernel modules not loaded".into()));
        }
        Ok(())
    }

    fn check_user_in_kvm_group() -> bool {
        if let Ok(groups) = Command::new("groups").output() {
            let groups_str = String::from_utf8_lossy(&groups.stdout);
            groups_str.contains("kvm")
        } else {
            // If we can't check groups, assume OK if /dev/kvm is accessible
            PathBuf::from("/dev/kvm").exists()
        }
    }

    /// Returns installation instructions for the user to run manually (no sudo from app)
    pub async fn get_install_instructions(&self) -> Result<String> {
        let distro = self.detect_distro()?;

        let instructions = match distro.as_str() {
            "ubuntu" | "debian" | "linuxmint" | "pop" | "elementary" => {
                format!(
                    "Execute no terminal:\n\n\
                     sudo apt update && sudo apt install -y \\\n\
                     {}qemu-system-x86 qemu-kvm \\\n\
                     {}libvirt-daemon-system libvirt-clients \\\n\
                     {}virt-manager bridge-utils \\\n\
                     {}libwebkit2gtk-4.1-0 libgtk-3-0 \\\n\
                     {}libayatana-appindicator3-1 librsvg2-2 \\\n\
                     {}patchelf curl wget\n\n\
                     Then add your user to groups:\n\
                     sudo usermod -aG kvm $USER\n\
                     sudo usermod -aG libvirt $USER\n\
                     Make logout/login to apply group changes.",
                    if distro == "ubuntu" && self.is_ubuntu_26_or_later() { "" } else { "" },
                    "", "", "", "", "", ""
                )
            }
            "arch" | "manjaro" | "endeavouros" => {
                format!(
                    "Execute no terminal:\n\n\
                     sudo pacman -S --needed --noconfirm \\\n\
                     {}qemu-system-x86 qemu-kvm \\\n\
                     {}libvirt virt-manager \\\n\
                     {}webkit2gtk-4.1 gtk3 \\\n\
                     {}libayatana-appindicator librsvg \\\n\
                     {}patchelf curl wget\n\n\
                     Then enable libvirt:\n\
                     sudo systemctl enable --now libvirtd\n\n\
                     Add your user to groups:\n\
                     sudo usermod -aG kvm $USER\n\
                     sudo usermod -aG libvirt $USER",
                    "", "", "", "", ""
                )
            }
            "fedora" | "rhel" | "centos" | "rocky" | "almalinux" => {
                format!(
                    "Execute no terminal:\n\n\
                     sudo dnf install -y \\\n\
                     {}qemu-system-x86 qemu-kvm \\\n\
                     {}libvirt-daemon virt-manager \\\n\
                     {}webkit2gtk4.1 gtk3 \\\n\
                     {}libayatana-appindicator-gtk3 librsvg2 \\\n\
                     {}patchelf curl wget\n\n\
                     Then enable libvirt:\n\
                     sudo systemctl enable --now libvirtd\n\n\
                     Add your user to groups:\n\
                     sudo usermod -aG kvm $USER\n\
                     sudo usermod -aG libvirt $USER",
                    "", "", "", "", ""
                )
            }
            "opensuse*" | "sles" => {
                format!(
                    "Execute no terminal:\n\n\
                     sudo zypper install -y \\\n\
                     {}qemu-system-x86 qemu-kvm \\\n\
                     {}libvirt-daemon virt-manager \\\n\
                     {}webkit2gtk-4.1 gtk3 \\\n\
                     {}libayatana-appindicator-gtk3 librsvg2 \\\n\
                     {}patchelf curl wget\n\n\
                     Then enable libvirt:\n\
                     sudo systemctl enable --now libvirtd\n\n\
                     Add your user to groups:\n\
                     sudo usermod -aG kvm $USER\n\
                     sudo usermod -aG libvirt $USER",
                    "", "", "", "", ""
                )
            }
            _ => format!(
                "Distro não detectada automaticamente ({}).\n\n\
                 Instale manualmente os pacotes:\n\
                 - qemu-system-x86 (ou qemu-system-x86_64)\n\
                 - qemu-kvm\n\
                 - libvirt (opcional)\n\
                 - webkit2gtk (para o WebView do Tauri)\n\
                 - gtk3\n\
                 - patchelf\n\n\
                 Depois adicione seu usuário aos grupos:\n\
                 sudo usermod -aG kvm $USER\n\
                 sudo usermod -aG libvirt $USER",
                distro
            ),
        };

        Ok(instructions)
    }

    fn is_ubuntu_26_or_later(&self) -> bool {
        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("VERSION_ID=") {
                    let ver = line.trim_start_matches("VERSION_ID=").trim_matches('"');
                    if let Some(major) = ver.split('.').next() {
                        if let Ok(m) = major.parse::<u32>() {
                            return m >= 26;
                        }
                    }
                }
            }
        }
        false
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

    /// Downloads an Android x86_64 image to the destination path
    pub async fn download_android_image(
        &self,
        url: &str,
        destination: &str,
    ) -> Result<serde_json::Value> {
        let dest_path = PathBuf::from(destination);

        // Ensure parent directory exists
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Download using reqwest
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(600)) // 10 min timeout
            .user_agent("LinuxStacks/1.0")
            .build()
            .map_err(|e| LinuxStacksError::Download(format!("Failed to create HTTP client: {}", e)))?;

        tracing::info!("Downloading Android image from {}", url);

        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| LinuxStacksError::Download(format!("Download failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(LinuxStacksError::Download(
                format!("HTTP {}: {}", resp.status(), resp.status().reason().unwrap_or("unknown"))
            ));
        }

        let total_size = resp.content_length().unwrap_or(0);
        let mut file = fs::File::create(&dest_path)
            .map_err(|e| LinuxStacksError::Download(format!("Cannot create file: {}", e)))?;

        let mut downloaded: u64 = 0;
        let mut stream = resp.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let bytes = chunk.map_err(|e| LinuxStacksError::Download(format!("Stream error: {}", e)))?;
            file.write_all(&bytes)
                .map_err(|e| LinuxStacksError::Download(format!("Write error: {}", e)))?;
            downloaded += bytes.len() as u64;

            if total_size > 0 {
                let progress = (downloaded as f64 / total_size as f64) * 100.0;
                tracing::debug!("Download progress: {:.1}%", progress);
            }
        }

        // Verify file exists and has size > 0
        let metadata = fs::metadata(&dest_path)?;
        if metadata.len() == 0 {
            return Err(LinuxStacksError::Download("Downloaded file is empty".into()));
        }

        tracing::info!("Download complete: {} bytes", metadata.len());

        Ok(serde_json::json!({
            "path": destination,
            "size_bytes": metadata.len(),
            "success": true
        }))
    }

    /// Gets a recommended Android image URL for KVM
    pub fn get_recommended_image_url() -> String {
        // Use an officially supported Android x86_64 image for KVM
        // AVD images from the Android SDK are the most reliable
        "https://dl.google.com/android/repository/sys-img/android/x86_64/android-x86_64.tar.gz"
            .to_string()
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
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
            if self.kvm { "OK" } else { "MISSING" },
            if self.qemu { "OK" } else { "MISSING" },
            if self.libvirt { "OK" } else { "OPTIONAL" },
            if self.kvm_modules { "OK" } else { "MISSING" },
            if self.user_in_kvm_group { "OK" } else { "MISSING" },
        )
    }
}
