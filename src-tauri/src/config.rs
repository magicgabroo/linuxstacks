use serde::{Deserialize, Serialize};
use config::{Config, File, Environment};
use dirs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    pub memory_mb: u32,
    pub cpus: u32,
    pub gpu: String,
    pub network: String,
    pub disk_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidConfig {
    pub image_url: String,
    pub shared_folder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub language: String,
    pub show_toolbar: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub f_droid: bool,
    pub aurora_store: bool,
    pub adb_integration: bool,
    pub clipboard_sync: bool,
    pub file_share: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub vm: VmConfig,
    pub android: AndroidConfig,
    pub ui: UiConfig,
    pub features: FeaturesConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/user"));
        let linuxstacks_dir = home.join("LinuxStacks");
        
        Self {
            vm: VmConfig {
                memory_mb: 4096,
                cpus: 4,
                gpu: "virtio-gpu".to_string(),
                network: "user-mode".to_string(),
                disk_path: linuxstacks_dir.join("android.qcow2").to_string_lossy().to_string(),
            },
            android: AndroidConfig {
                image_url: "https://dl.google.com/android/repository/android-x86_64.img".to_string(),
                shared_folder: linuxstacks_dir.join("shared").to_string_lossy().to_string(),
            },
            ui: UiConfig {
                theme: "auto".to_string(),
                language: "pt-BR".to_string(),
                show_toolbar: true,
            },
            features: FeaturesConfig {
                f_droid: true,
                aurora_store: true,
                adb_integration: true,
                clipboard_sync: true,
                file_share: true,
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, crate::error::LinuxStacksError> {
        let config_path = Self::config_path()?;
        
        let builder = Config::builder()
            .add_source(File::with_name(&config_path.to_string_lossy()).required(false))
            .add_source(Environment::with_prefix("LINUXSTACKS").separator("__"))
            .add_source(File::with_name("linuxstacks.toml").required(false));
        
        let config: AppConfig = builder.build()?.try_deserialize()?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<(), crate::error::LinuxStacksError> {
        let config_path = Self::config_path()?;
        let toml_str = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, toml_str)?;
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf, crate::error::LinuxStacksError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::error::LinuxStacksError::Config("Config dir not found".into()))?
            .join("linuxstacks");
        
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("linuxstacks.toml"))
    }
    
    pub fn ensure_directories(&self) -> Result<(), crate::error::LinuxStacksError> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/user"));
        let linuxstacks_dir = home.join("LinuxStacks");
        
        std::fs::create_dir_all(&linuxstacks_dir)?;
        std::fs::create_dir_all(linuxstacks_dir.join("shared"))?;
        std::fs::create_dir_all(linuxstacks_dir.join("images"))?;
        std::fs::create_dir_all(linuxstacks_dir.join("snapshots"))?;
        
        Ok(())
    }
}