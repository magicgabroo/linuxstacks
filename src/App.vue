<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useVmStore } from '@/stores/vmStore'
import Sidebar from '@/components/Sidebar.vue'
import Toolbar from '@/components/Toolbar.vue'
import SettingsPanel from '@/components/SettingsPanel.vue'

const vmStore = useVmStore()
const showSettings = ref(false)
const showSidebar = ref(true)

const vmStatusClass = computed(() => {
  if (vmStore.vmStatus.running) return 'text-green-500'
  if (vmStore.vmStatus.pid) return 'text-yellow-500'
  return 'text-gray-500'
})

const vmStatusText = computed(() => {
  if (vmStore.vmStatus.running) return 'Executando'
  if (vmStore.vmStatus.pid) return 'Parando...'
  return 'Parado'
})

async function startVm() {
  try {
    await vmStore.startVm(vmStore.config.android.image_url || undefined)
  } catch (error) {
    console.error('Failed to start VM:', error)
  }
}

async function stopVm() {
  try {
    await vmStore.stopVm()
  } catch (error) {
    console.error('Failed to stop VM:', error)
  }
}

async function restartVm() {
  await stopVm()
  await new Promise(resolve => setTimeout(resolve, 2000))
  await startVm()
}

onMounted(async () => {
  vmStore.fetchVmStatus()
  vmStore.loadConfig()

  const tauri = (window as any).__TAURI__
  if (tauri) {
    const { listen } = await import('@tauri-apps/api/event')

    const unlistenStarted = await listen('vm-started', (event: any) => {
      vmStore.vmStatus.running = true
      vmStore.vmStatus.pid = event?.payload?.pid
    })
    const unlistenStopped = await listen('vm-stopped', () => {
      vmStore.vmStatus.running = false
      vmStore.vmStatus.pid = undefined
    })
    const unlistenError = await listen('vm-error', (event: any) => {
      vmStore.vmStatus.running = false
      console.error('VM Error:', event?.payload)
    })
    const unlistenPrereq = await listen('prerequisites-missing', () => {
      showSettings.value = true
    })

    onUnmounted(() => {
      unlistenStarted()
      unlistenStopped()
      unlistenError()
      unlistenPrereq()
    })
  }
})
</script>

<template>
  <div class="flex h-screen w-screen bg-gray-950 overflow-hidden">
    <!-- Sidebar -->
    <Sidebar
      v-model:open="showSidebar"
      @open-settings="showSettings = true"
      class="transition-transform duration-300 ease-in-out z-20"
    />

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Toolbar -->
      <Toolbar
        :vm-running="vmStore.vmStatus.running"
        :vm-pid="vmStore.vmStatus.pid"
        :show-sidebar="showSidebar"
        @toggle-sidebar="showSidebar = !showSidebar"
        @toggle-settings="showSettings = !showSettings"
        @start-vm="startVm"
        @stop-vm="stopVm"
        @restart-vm="restartVm"
      />

      <!-- VM Window Area -->
      <div class="flex-1 relative overflow-hidden bg-gray-950">
        <!-- VM Iframe/View -->
        <div class="absolute inset-0">
          <iframe
            v-if="vmStore.vmStatus.running"
            class="w-full h-full border-0"
            src="http://localhost:5555"
            allow="accelerometer; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
          />

          <!-- Placeholder when VM not running -->
          <div
            v-if="!vmStore.vmStatus.running"
            class="absolute inset-0 flex flex-col items-center justify-center bg-gray-950 p-8"
          >
            <div class="text-center max-w-md">
              <div class="w-24 h-24 mx-auto mb-6">
                <svg class="w-full h-full text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                    d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
                </svg>
              </div>
              <h2 class="text-2xl font-semibold text-white mb-2">LinuxStacks</h2>
              <p class="text-gray-400 mb-6">
                Emulador Android nativo para Linux usando KVM/QEMU.
                Configure a imagem Android e clique em "Iniciar VM" para começar.
              </p>
              <div class="flex justify-center gap-4">
                <button
                  @click="startVm"
                  class="btn btn-primary px-8 py-3 text-lg"
                >
                  Iniciar VM
                </button>
                <button
                  @click="showSettings = true"
                  class="btn btn-secondary px-8 py-3 text-lg"
                >
                  Configurações
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Settings Panel -->
      <SettingsPanel
        v-model:open="showSettings"
        @close="showSettings = false"
      />
    </div>
  </div>
</template>

<style scoped>
/* VM Window styles handled by Tailwind */
</style>
