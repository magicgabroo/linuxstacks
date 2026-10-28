use std::sync::Mutex;
use tauri::Manager;
use config::AppConfig;
use vm_manager::VmManager;
use installer::Installer;

/// Tracks currently running QEMU process PIDs
type RunningVms = Mutex<Vec<u32>>;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::load().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config, using defaults: {}", e);
        AppConfig::default()
    });

    if let Err(e) = config.ensure_directories() {
        tracing::error!("Failed to create directories: {}", e);
    }

    let vm_manager = VmManager::new(config.vm.clone());
    let installer = Installer::new(config.clone());

    tauri::Builder::default()
        .setup(|app| {
            app.manage(vm_manager);
            app.manage(installer);
            app.manage(config);
            app.manage(RunningVms::new(Vec::new()));

            // Check prerequisites after UI loads
            let app_handle = app.handle().clone();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                if let Ok(check) = Installer::check_prerequisites() {
                    if !check.all_ok {
                        let _ = app_handle.emit_all("prerequisites-missing", &check);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_vm,
            stop_vm,
            get_vm_status,
            create_disk_image,
            resize_disk_image,
            get_config,
            save_config,
            check_prerequisites,
            install_dependencies,
            download_android_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ---------------------------------------------------------------------------
// Helper: filter alive PIDs from the tracking list
// ---------------------------------------------------------------------------
fn filter_alive_pids(pids: &mut Vec<u32>) {
    pids.retain(|&pid| {
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
    });
}

// ---------------------------------------------------------------------------
// VM Commands
// ---------------------------------------------------------------------------

#[tauri::command]
async fn start_vm(
    vm_manager: tauri::State<'_, VmManager>,
    running_vms: tauri::State<'_, RunningVms>,
    app: tauri::AppHandle,
    android_img: Option<String>,
) -> Result<u32, String> {
    let img_path = android_img.as_deref();

    // Clean up dead PIDs first
    {
        let mut pids = running_vms.lock().unwrap();
        filter_alive_pids(&mut *pids);
        // If one is still running, return its PID
        if let Some(&pid) = pids.first() {
            return Ok(pid);
        }
    }

    let pid = vm_manager.start_vm(img_path).await.map_err(|e| e.to_string())?;

    running_vms.lock().unwrap().push(pid);
    let _ = app.emit_all("vm-started", &serde_json::json!({ "pid": pid }));

    Ok(pid)
}

#[tauri::command]
async fn stop_vm(
    vm_manager: tauri::State<'_, VmManager>,
    running_vms: tauri::State<'_, RunningVms>,
    app: tauri::AppHandle,
    pid: u32,
) -> Result<(), String> {
    vm_manager.stop_vm(pid).await.map_err(|e| e.to_string())?;

    let mut pids = running_vms.lock().unwrap();
    pids.retain(|&p| p != pid);

    let _ = app.emit_all("vm-stopped", &serde_json::json!({ "pid": pid }));
    Ok(())
}

#[tauri::command]
fn get_vm_status(
    vm_manager: tauri::State<'_, VmManager>,
    running_vms: tauri::State<'_, RunningVms>,
) -> serde_json::Value {
    let mut pids = running_vms.lock().unwrap();
    filter_alive_pids(&mut *pids);

    let status = vm_manager.get_vm_status();

    serde_json::json!({
        "running": !pids.is_empty(),
        "pid": pids.first().copied(),
        "memory_mb": status.memory_mb,
        "cpu_percent": 0.0
    })
}

#[tauri::command]
async fn create_disk_image(
    vm_manager: tauri::State<'_, VmManager>,
    size_gb: u32,
) -> Result<(), String> {
    vm_manager.create_disk_image(size_gb).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn resize_disk_image(
    vm_manager: tauri::State<'_, VmManager>,
    size_gb: u32,
) -> Result<(), String> {
    let disk_path = vm_manager.disk_path().to_string();
    if !std::path::PathBuf::from(&disk_path).exists() {
        return Err("Disk image does not exist. Create it first with create_disk_image.".into());
    }

    let output = tokio::process::Command::new("qemu-img")
        .args(["resize", &disk_path, &format!("{}G", size_gb)])
        .output()
        .await
        .map_err(|e| format!("Failed to resize disk: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("qemu-img resize failed: {}", stderr));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Config Commands
// ---------------------------------------------------------------------------

#[tauri::command]
fn get_config(config: tauri::State<'_, AppConfig>) -> AppConfig {
    config.inner().clone()
}

#[tauri::command]
fn save_config(
    config: tauri::State<'_, AppConfig>,
    new_config: AppConfig,
) -> Result<(), String> {
    new_config.save().map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Installer Commands
// ---------------------------------------------------------------------------

#[tauri::command]
fn check_prerequisites() -> Result<installer::PrerequisiteCheck, String> {
    Installer::check_prerequisites().map_err(|e| e.to_string())
}

#[tauri::command]
async fn install_dependencies(
    installer: tauri::State<'_, Installer>,
) -> Result<String, String> {
    installer.get_install_instructions().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn download_android_image(
    installer: tauri::State<'_, Installer>,
    url: String,
    destination: String,
) -> Result<serde_json::Value, String> {
    installer.download_android_image(&url, &destination).await
}
