use std::env;

fn main() {
    // Embed version info
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=APP_VERSION={}", version);
    
    // Tauri build
    tauri_build::build()
}