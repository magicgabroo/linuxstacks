#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod vm_manager;
mod installer;
mod ipc;
mod error;

use tauri::Manager;
use config::AppConfig;
use vm_manager::VmManager;
use installer::Installer;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    // Load configuration
    let config = AppConfig::load().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config, using defaults: {}", e);
        AppConfig::default()
    });
    
    // Ensure directories exist
    if let Err(e) = config.ensure_directories() {
        tracing::error!("Failed to create directories: {}", e);
    }
    
    // Create VM manager
    let vm_manager = VmManager::new(config.vm.clone());
    
    // Create installer
    let installer = Installer::new(config.clone());
    
    tauri::Builder::default()
        .setup(|app| {
            // Set up global state
            app.manage(vm_manager);
            app.manage(installer);
            app.manage(config);
            
            // Set up IPC handlers
            setup_ipc_handlers(app)?;
            
            // Check prerequisites on startup
            check_prerequisites_on_startup(app.handle().clone());
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // VM commands
            start_vm,
            stop_vm,
            get_vm_status,
            create_disk_image,
            
            // Config
            get_config,
            save_config,
            
            // Installer
            check_prerequisites,
            install_dependencies,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_ipc_handlers(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Set up event listeners for backend -> frontend communication
    let app_handle = app.handle().clone();
    
    // Listen for VM events from backend
    // In a real implementation, you'd set up channels here
    
    Ok(())
}

fn check_prerequisites_on_startup(app_handle: tauri::AppHandle) {
    tokio::spawn(async move {
        // Give UI time to load
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        
        // Check prerequisites
        if let Ok(check) = installer::Installer::check_prerequisites() {
            if !check.all_ok {
                // Emit event to show setup wizard
                let _ = app_handle.emit_all("prerequisites-missing", check);
            }
        }
    }
}

// Tauri commands
#[tauri::command]
async fn start_vm(vm_manager: tauri::State<'_, crate::vm_manager::VmManager>) -> Result<u32, String> {
    vm_manager.start_vm().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_vm(vm_manager: tauri::State<'_, crate::vm_manager::VmManager>, pid: u32) -> Result<(), String> {
    vm_manager.stop_vm(pid).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_vm_status(vm_manager: tauri::State<'_, crate::vm_manager::VmManager>) -> Result<crate::vm_manager::VmStatus, String> {
    vm_manager.get_vm_status().map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_disk_image(vm_manager: tauri::State<'_, crate::vm_manager::VmManager>, size_gb: u32) -> Result<(), String> {
    vm_manager.create_disk_image(size_gb).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn get_config(config: tauri::State<'_, crate::config::AppConfig>) -> crate::config::AppConfig {
    config.inner().clone()
}

#[tauri::command]
fn save_config(config: tauri::State<'_, crate::config::AppConfig>, new_config: crate::config::AppConfig) -> Result<(), String> {
    new_config.save().map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_prerequisites() -> Result<crate::installer::PrerequisiteCheck, String> {
    crate::installer::Installer::check_prerequisites().map_err(|e| e.to_string())
}

#[tauri::command]
async fn install_dependencies(installer: tauri::State<'_, crate::installer::Installer>) -> Result<(), String> {
    installer.install_dependencies().await.map_err(|e| e.to_string())
}