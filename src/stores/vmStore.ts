import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { AppConfig, VmConfig, VmStatus } from '@/types'

export const useVmStore = defineStore('vm', () => {
  // State
  const config = ref<AppConfig>({
    vm: {
      memory_mb: 4096,
      cpus: 4,
      gpu: 'virtio-gpu',
      network: 'user-mode',
      disk_path: '~/LinuxStacks/android.qcow2',
    },
    android: {
      image_url: 'https://dl.google.com/android/repository/android-x86_64.img',
      shared_folder: '~/LinuxStacks/shared',
    },
    ui: {
      theme: 'auto',
      language: 'pt-BR',
      show_toolbar: true,
    },
    features: {
      f_droid: true,
      aurora_store: true,
      adb_integration: true,
      clipboard_sync: true,
      file_share: true,
    },
  })
  
  const vmStatus = ref<VmStatus>({
    running: false,
    pid: undefined,
    memory_mb: 4096,
    cpu_percent: 0,
  })
  
  const loading = ref(false)
  const error = ref<string | null>(null)
  
  // Getters
  const isVmRunning = computed(() => vmStatus.value.running)
  const vmPid = computed(() => vmStore.value.vmStatus.pid)
  
  // Actions
  async function loadConfig() {
    try {
      if (window.__TAURI__) {
        const savedConfig = await window.__TAURI__.invoke('get_config')
        if (savedConfig) {
          config.value = savedConfig
        }
      }
    } catch (error) {
      console.error('Failed to load config:', error)
    }
  }
  
  async function saveConfig() {
    try {
      if (window.__TAURI__) {
        await window.__TAURI__.invoke('save_config', { config: config.value })
      }
    } catch (error) {
      console.error('Failed to save config:', error)
      throw error
    }
  }
  
  async function fetchVmStatus() {
    try {
      if (window.__TAURI__) {
        const status = await window.__TAURI__.invoke('get_vm_status')
        if (status) {
          vmStatus.value = status
        }
      }
    } catch (error) {
      console.error('Failed to fetch VM status:', error)
    }
  }
  
  async function startVm() {
    loading.value = true
    error.value = null
    try {
      if (window.__TAURI__) {
        const pid = await window.__TAURI__.invoke('start_vm')
        vmStatus.value.running = true
        vmStatus.value.pid = pid
      }
    } catch (err: any) {
      error.value = err.message || 'Failed to start VM'
      throw error
    } finally {
      loading.value = false
    }
  }
  
  async function stopVm() {
    loading.value = true
    error.value = null
    try {
      if (window.__TAURI__ && vmStatus.value.pid) {
        await window.__TAURI__.invoke('stop_vm', { pid: vmStatus.value.pid })
        vmStatus.value.running = false
        vmStatus.value.pid = undefined
      }
    } catch (err: any) {
      error.value = err.message || 'Failed to stop VM'
      throw error
    } finally {
      loading.value = false
    }
  }
  
  async function resizeDisk(sizeGb: number) {
    loading.value = true
    error.value = null
    try {
      if (window.__TAURI__) {
        await window.__TAURI__.invoke('create_disk_image', { sizeGb })
      }
    } catch (err: any) {
      error.value = err.message || 'Failed to resize disk'
      throw error
    } finally {
      loading.value = false
    }
  }
  
  async function checkPrerequisites() {
    try {
      if (window.__TAURI__) {
        return await window.__TAURI__.invoke('check_prerequisites')
      }
    } catch (error) {
      console.error('Failed to check prerequisites:', error)
      throw error
    }
  }
  
  async function installDependencies() {
    try {
      if (window.__TAURI__) {
        await window.__TAURI__.invoke('install_dependencies')
      }
    } catch (error) {
      console.error('Failed to install dependencies:', error)
      throw error
    }
  }
  
  return {
    config,
    vmStatus,
    loading,
    error,
    isVmRunning,
    vmPid,
    loadConfig,
    saveConfig,
    fetchVmStatus,
    startVm,
    stopVm,
    resizeDisk,
    checkPrerequisites,
    installDependencies,
  }
})