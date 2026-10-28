import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { AppConfig, VmStatus, PrerequisiteCheck } from '@/types'

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
  const prerequisites = ref<PrerequisiteCheck | null>(null)

  // Getters
  const isVmRunning = computed(() => vmStatus.value.running)
  const vmPid = computed(() => vmStatus.value.pid)
  const hasError = computed(() => error.value !== null)

  // Actions
  async function loadConfig() {
    try {
      const savedConfig = await invoke<AppConfig>('get_config')
      if (savedConfig) {
        config.value = { ...config.value, ...savedConfig }
      }
    } catch (err) {
      console.error('Failed to load config:', err)
    }
  }

  async function saveConfig() {
    try {
      loading.value = true
      error.value = null
      await invoke('save_config', { config: config.value })
    } catch (err: any) {
      error.value = err?.message || 'Failed to save config'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function fetchVmStatus() {
    try {
      const status = await invoke<Record<string, unknown>>('get_vm_status')
      if (status) {
        vmStatus.value = {
          running: status.running as boolean,
          pid: status.pid as number | undefined,
          memory_mb: (status.memory_mb as number) || vmStatus.value.memory_mb,
          cpu_percent: (status.cpu_percent as number) || 0,
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
      const pid = await invoke<number>('start_vm', { androidImg: androidImgPath ?? null })
      vmStatus.value.running = true
      vmStatus.value.pid = pid
      return pid
    } catch (err: any) {
      error.value = err?.message || 'Failed to start VM'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function stopVm() {
    loading.value = true
    error.value = null
    try {
      if (vmStatus.value.pid) {
        await invoke('stop_vm', { pid: vmStatus.value.pid })
      }
      vmStatus.value.running = false
      vmStatus.value.pid = undefined
    } catch (err: any) {
      error.value = err?.message || 'Failed to stop VM'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function resizeDisk(sizeGb: number) {
    loading.value = true
    error.value = null
    try {
      await invoke('resize_disk_image', { sizeGb })
    } catch (err: any) {
      error.value = err?.message || 'Failed to resize disk'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function checkPrerequisites(): Promise<PrerequisiteCheck | null> {
    try {
      const check = await invoke<Record<string, unknown>>('check_prerequisites')
      const result: PrerequisiteCheck = {
        kvm: Boolean(check.kvm),
        qemu: Boolean(check.qemu),
        libvirt: Boolean(check.libvirt),
        kvm_modules: Boolean(check.kvm_modules),
        user_in_kvm_group: Boolean(check.user_in_kvm_group),
        all_ok: Boolean(check.all_ok),
      }
      prerequisites.value = result
      return result
    } catch (err) {
      console.error('Failed to check prerequisites:', err)
      return null
    }
  }

  async function installDependencies(): Promise<string> {
    try {
      return await invoke<string>('install_dependencies')
    } catch (err) {
      console.error('Failed to get install instructions:', err)
      throw err
    }
  }

  async function downloadAndroidImage(url: string, destination: string): Promise<Record<string, unknown>> {
    try {
      return await invoke<Record<string, unknown>>('download_android_image', { url, destination })
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
    prerequisites,
    isVmRunning,
    vmPid,
    hasError,
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
