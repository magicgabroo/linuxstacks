use std::path::PathBuf;
use tokio::process::Command as TokioCommand;
use serde::{Deserialize, Serialize};
use crate::config::VmConfig;
use crate::error::{LinuxStacksError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub memory_mb: u32,
    pub cpu_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInfo {
    pub name: String,
    pub disk_path: String,
    pub memory_mb: u32,
    pub cpus: u32,
    pub status: VmStatus,
}

pub struct VmManager {
    config: VmConfig,
    disk_path: PathBuf,
}

impl VmManager {
    pub fn new(config: VmConfig) -> Self {
        let disk_path = PathBuf::from(&config.disk_path);
        Self { config, disk_path }
    }

    pub fn disk_path(&self) -> &str {
        &self.config.disk_path
    }

    pub fn check_kvm_available() -> Result<bool> {
        let kvm_path = PathBuf::from("/dev/kvm");
        if !kvm_path.exists() {
            return Err(LinuxStacksError::KvmUnavailable(
                "/dev/kvm not found. Enable virtualization in BIOS and ensure user is in 'kvm' group".into(),
            ));
        }

        use std::os::unix::fs::MetadataExt;
        let metadata = std::fs::metadata("/dev/kvm")
            .map_err(|e| LinuxStacksError::KvmUnavailable(format!("Cannot access /dev/kvm: {}", e)))?;

        let mode = metadata.mode();
        if mode & 0o444 == 0 {
            return Err(LinuxStacksError::KvmUnavailable(
                "/dev/kvm is not readable. Add user to 'kvm' group: sudo usermod -aG kvm $USER".into(),
            ));
        }

        Ok(true)
    }

    pub fn check_qemu_installed() -> Result<()> {
        let output = std::process::Command::new("qemu-system-x86_64")
            .arg("--version")
            .output()
            .map_err(|_| LinuxStacksError::Vm(
                "qemu-system-x86_64 not found. Install with: sudo apt install qemu-system-x86".into(),
            ))?;

        if !output.status.success() {
            return Err(LinuxStacksError::Vm("qemu-system-x86_64 failed to run".into()));
        }
        Ok(())
    }

    pub async fn create_disk_image(&self, size_gb: u32) -> Result<()> {
        let disk_path = &self.config.disk_path;

        if PathBuf::from(disk_path).exists() {
            return Ok(());
        }

        if let Some(parent) = PathBuf::from(disk_path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| LinuxStacksError::Vm(format!("Cannot create parent dir: {}", e)))?;
        }

        let output = TokioCommand::new("qemu-img")
            .args([
                "create",
                "-f",
                "qcow2",
                disk_path,
                &format!("{}G", size_gb),
            ])
            .output()
            .await
            .map_err(|e| LinuxStacksError::Vm(format!("Failed to create disk image: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(LinuxStacksError::Vm(format!("qemu-img failed: {}", stderr)));
        }

        Ok(())
    }

    pub fn build_qemu_args(&self, android_img: Option<&str>) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "-enable-kvm".to_string(),
            "-m".to_string(),
            self.config.memory_mb.to_string(),
            "-cpu".to_string(),
            "host".to_string(),
            "-smp".to_string(),
            self.config.cpus.to_string(),
            "-drive".to_string(),
            format!(
                "file={},format=qcow2,if=virtio",
                self.config.disk_path
            ),
        ];

        // Android disk image (bootable raw image)
        if let Some(img) = android_img {
            args.extend([
                "-drive".to_string(),
                format!("file={},format=raw,if=none,id=android", img),
                "-device".to_string(),
                "virtio-blk-pci,drive=android".to_string(),
            ]);
        }

        // Network
        args.extend([
            "-netdev".to_string(),
            "user,id=net0,hostfwd=tcp::5555-:5555,hostfwd=tcp::5556-:5556".to_string(),
            "-device".to_string(),
            "virtio-net-pci,netdev=net0".to_string(),
        ]);

        // GPU
        args.extend([
            "-display".to_string(),
            "gtk,gl=on".to_string(),
            "-device".to_string(),
            "virtio-gpu-pci".to_string(),
        ]);

        // Input
        args.extend([
            "-device".to_string(),
            "usb-kbd".to_string(),
            "-device".to_string(),
            "usb-tablet".to_string(),
        ]);

        // Name and UUID
        args.extend([
            "-name".to_string(),
            "LinuxStacks".to_string(),
            "-uuid".to_string(),
            uuid::Uuid::new_v4().to_string(),
        ]);

        args
    }

    pub async fn start_vm(&self, android_img_path: Option<&str>) -> Result<u32> {
        // Ensure disk exists
        self.create_disk_image(32).await?;

        let args = self.build_qemu_args(android_img_path);

        let mut cmd = TokioCommand::new("qemu-system-x86_64");
        cmd.args(&args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .stdin(std::process::Stdio::null());

        let child = cmd
            .spawn()
            .map_err(|e| LinuxStacksError::Vm(format!("Failed to spawn QEMU: {}", e)))?;

        let pid = child.id().ok_or_else(|| {
            LinuxStacksError::Vm("Failed to get QEMU PID".into())
        })?;

        // Monitor process in background and clean up on exit
        let _handle = tokio::spawn(async move {
            let _ = child.wait().await;
        });

        Ok(pid)
    }

    pub async fn stop_vm(&self, pid: u32) -> Result<()> {
        #[cfg(unix)]
        {
            let status = std::process::Command::new("kill")
                .args(["-TERM", &pid.to_string()])
                .output()
                .map_err(|e| LinuxStacksError::Vm(format!("Failed to send SIGTERM: {}", e)))?;

            if !status.status.success() {
                tracing::warn!("Failed to stop VM pid={}: {}", pid, String::from_utf8_lossy(&status.stderr));
            }
        }

        Ok(())
    }

    pub fn get_vm_status(&self) -> VmStatus {
        VmStatus {
            running: false,
            pid: None,
            memory_mb: self.config.memory_mb,
            cpu_percent: 0.0,
        }
    }

    pub fn is_process_running(pid: u32) -> bool {
        #[cfg(unix)]
        {
            std::process::Command::new("kill")
                .args(["-0", &pid.to_string()])
                .output()
                .map(|out| out.status.success())
                .unwrap_or(false)
        }

        #[cfg(not(unix))]
        {
            false
        }
    }
}
