use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IpcMessage {
    // VM commands
    StartVm,
    StopVm { pid: u32 },
    GetVmStatus,
    CreateDiskImage { size_gb: u32 },
    
    // Config
    GetConfig,
    SaveConfig { config: crate::config::AppConfig },
    GetPrerequisites,
    
    // Installer
    CheckPrerequisites,
    InstallDependencies,
    
    // Events (backend -> frontend)
    VmStarted { pid: u32 },
    VmStopped,
    VmError { message: String },
    InstallProgress { step: String, progress: f32 },
    InstallComplete { success: bool, message: String },
    PrerequisiteCheck { check: crate::installer::PrerequisiteCheck },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: String,
    pub message: IpcMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

pub fn handle_ipc_message(
    message: IpcMessage,
    vm_manager: tauri::State<'_, crate::vm_manager::VmManager>,
    installer: tauri::State<'_, crate::installer::Installer>,
    config: tauri::State<'_, crate::config::AppConfig>,
) -> Result<IpcResponse> {
    let id = uuid::Uuid::new_v4().to_string();
    
    match message {
        IpcMessage::StartVm => {
            let vm_manager = vm_manager.inner().clone();
            let config = config.inner().clone();
            
            tokio::spawn(async move {
                let vm_manager = crate::vm_manager::VmManager::new(config.vm.clone());
                if let Ok(pid) = vm_manager.start_vm().await {
                    // Emit event to frontend
                    // In real implementation, use tauri::Emitter
                }
            });
            
            Ok(IpcResponse {
                id,
                success: true,
                data: Some(serde_json::json!({ "message": "VM starting..." })),
                error: None,
            })
        }
        
        IpcMessage::StopVm { pid } => {
            let vm_manager = crate::vm_manager::VmManager::new(config.inner().vm.clone());
            let result = tokio::runtime::Handle::current().block_on(vm_manager.stop_vm(pid));
            
            match result {
                Ok(_) => Ok(IpcResponse {
                    id,
                    success: true,
                    data: Some(serde_json::json!({ "message": "VM stopped" })),
                    error: None,
                }),
                Err(e) => Ok(IpcResponse {
                    id,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                }),
            }
        }
        
        IpcMessage::GetVmStatus => {
            let vm_manager = crate::vm_manager::VmManager::new(config.inner().vm.clone());
            match vm_manager.get_vm_status() {
                Ok(status) => Ok(IpcResponse {
                    id,
                    success: true,
                    data: Some(serde_json::to_value(status)?),
                    error: None,
                }),
                Err(e) => Ok(IpcResponse {
                    id,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                }),
            }
        }
        
        IpcMessage::GetConfig => {
            Ok(IpcResponse {
                id,
                success: true,
                data: Some(serde_json::to_value(config.inner().clone())?),
                error: None,
            })
        }
        
        IpcMessage::SaveConfig { config: new_config } => {
            if let Err(e) = new_config.save() {
                Ok(IpcResponse {
                    id,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                })
            } else {
                Ok(IpcResponse {
                    id,
                    success: true,
                    data: Some(serde_json::json!({ "message": "Config saved" })),
                    error: None,
                })
            }
        }
        
        IpcMessage::CheckPrerequisites => {
            match crate::installer::Installer::check_prerequisites() {
                Ok(check) => Ok(IpcResponse {
                    id,
                    success: true,
                    data: Some(serde_json::to_value(check)?),
                    error: None,
                }),
                Err(e) => Ok(IpcResponse {
                    id,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                }),
            }
        }
        
        IpcMessage::GetPrerequisites => {
            // Same as CheckPrerequisites
            handle_ipc_message(IpcMessage::CheckPrerequisites, vm_manager, installer, config)
        }
        
        IpcMessage::InstallDependencies => {
            let installer = installer.inner().clone();
            let config = config.inner().clone();
            
            tokio::spawn(async move {
                let installer = crate::installer::Installer::new(config.clone());
                if let Err(e) = installer.install_dependencies().await {
                    // Emit error event
                }
            });
            
            Ok(IpcResponse {
                id,
                success: true,
                data: Some(serde_json::json!({ "message": "Installation started" })),
                error: None,
            })
        }
        
        _ => Ok(IpcResponse {
            id,
            success: false,
            data: None,
            error: Some("Unknown message type".into()),
        }),
    }
}