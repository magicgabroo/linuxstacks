use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinuxStacksError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("VM error: {0}")]
    Vm(String),
    
    #[error("Installation error: {0}")]
    Install(String),
    
    #[error("Download error: {0}")]
    Download(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    Permission(String),
    
    #[error("KVM not available: {0}")]
    KvmUnavailable(String),
}

pub type Result<T> = std::result::Result<T, LinuxStacksError>;