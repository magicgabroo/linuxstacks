import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { AppConfig, VmConfig, VmStatus, PrerequisiteCheck } from '@/types'

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
      image_url: '',
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
  const vmPid = computed(() => vmStatus.value.pid)

  // Actions
  async function loadConfig() {
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        const savedConfig = await tauri.invoke('get_config')
        if (savedConfig) {
          config.value = { ...config.value, ...savedConfig }
        }
      }
    } catch (err) {
      console.error('Failed to load config:', err)
    }
  }

  async function saveConfig() {
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        await tauri.invoke('save_config', { config: config.value })
      }
    } catch (err) {
      console.error('Failed to save config:', err)
      throw err
    }
  }

  async function fetchVmStatus() {
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        const status = await tauri.invoke('get_vm_status')
        if (status) {
          vmStatus.value = { ...vmStatus.value, ...status }
        }
      }
    } catch (err) {
      console.error('Failed to fetch VM status:', err)
    }
  }

  async function startVm(androidImgPath?: string) {
    loading.value = true
    error.value = null
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        const pid = await tauri.invoke('start_vm', { androidImg: androidImgPath || null })
        vmStatus.value.running = true
        vmStatus.value.pid = pid
        return pid
      }
    } catch (err: any) {
      error.value = err?.message || 'Failed to start VM'
      throw error
    } finally {
      loading.value = false
    }
  }

  async function stopVm() {
    loading.value = true
    error.value = null
    try {
      const tauri = (window as any).__TAURI__
      if (tauri && vmStatus.value.pid) {
        await tauri.invoke('stop_vm', { pid: vmStatus.value.pid })
        vmStatus.value.running = false
        vmStatus.value.pid = undefined
      }
    } catch (err: any) {
      error.value = err?.message || 'Failed to stop VM'
      throw error
    } finally {
      loading.value = false
    }
  }

  async function resizeDisk(sizeGb: number) {
    loading.value = true
    error.value = null
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        await tauri.invoke('resize_disk_image', { sizeGb })
      }
    } catch (err: any) {
      error.value = err?.message || 'Failed to resize disk'
      throw error
    } finally {
      loading.value = false
    }
  }

  async function checkPrerequisites(): Promise<PrerequisiteCheck | null> {
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        return await tauri.invoke('check_prerequisites')
      }
    } catch (err) {
      console.error('Failed to check prerequisites:', err)
    }
    return null
  }

  async function installDependencies(): Promise<string> {
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        return await tauri.invoke('install_dependencies')
      }
    } catch (err) {
      console.error('Failed to get install instructions:', err)
      throw err
    }
    return ''
  }

  async function downloadAndroidImage(url: string, destination: string): Promise<any> {
    try {
      const tauri = (window as any).__TAURI__
      if (tauri) {
        return await tauri.invoke('download_android_image', { url, destination })
      }
    } catch (err) {
      console.error('Failed to download Android image:', err)
      throw err
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
    downloadAndroidImage,
  }
})
