// Type definitions for LinuxStacks

export interface VmConfig {
  memory_mb: number
  cpus: number
  gpu: string
  network: string
  disk_path: string
}

export interface AndroidConfig {
  image_url: string
  shared_folder: string
}

export interface UiConfig {
  theme: string
  language: string
  show_toolbar: boolean
}

export interface FeaturesConfig {
  f_droid: boolean
  aurora_store: boolean
  adb_integration: boolean
  clipboard_sync: boolean
  file_share: boolean
}

export interface AppConfig {
  vm: VmConfig
  android: AndroidConfig
  ui: UiConfig
  features: FeaturesConfig
}

export interface VmStatus {
  running: boolean
  pid?: number
  memory_mb: number
  cpu_percent: number
}

export interface PrerequisiteCheck {
  kvm: boolean
  qemu: boolean
  libvirt: boolean
  kvm_modules: boolean
  user_in_kvm_group: boolean
  all_ok: boolean
}

export interface DownloadResult {
  path: string
  size_bytes: number
  success: boolean
}
