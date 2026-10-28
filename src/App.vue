<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useVmStore } from '@/stores/vmStore'
import { listen } from '@tauri-apps/api/event'
import InstallWizard from './InstallWizard.vue'

const vmStore = useVmStore()
const showSettings = ref(false)
const showSidebar = ref(true)
const showWizard = ref(false)

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

async function checkAndShowWizard() {
  const check = await vmStore.checkPrerequisites()
  if (check && !check.all_ok) {
    showWizard.value = true
  }
}

onMounted(async () => {
  await vmStore.fetchVmStatus()
  await vmStore.loadConfig()

  // Listen for VM events
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

  // Check prerequisites after load
  await checkAndShowWizard()
})

function closeSettings() {
  showSettings.value = false
}
</script>

<template>
  <div class="flex h-screen w-screen bg-gray-950 overflow-hidden">
    <!-- Install Wizard -->
    <InstallWizard v-if="showWizard" @complete="showWizard = false" />

    <!-- Sidebar -->
    <Sidebar
      v-model="showSidebar"
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
        @toggle-settings="showSettings = true"
        @start-vm="startVm"
        @stop-vm="stopVm"
        @restart-vm="restartVm"
      />

      <!-- VM Window Area -->
      <div class="flex-1 relative overflow-hidden bg-gray-950">
        <div class="absolute inset-0">
          <!-- VM Display Placeholder -->
          <div
            v-if="vmStore.vmStatus.running"
            class="absolute inset-0 flex flex-col items-center justify-center bg-gray-900"
          >
            <div class="text-center max-w-md px-6">
              <div class="w-20 h-20 mx-auto mb-6">
                <svg class="w-full h-full text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                    d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
                </svg>
              </div>
              <h3 class="text-xl font-semibold text-white mb-2">VM em execução</h3>
              <p class="text-gray-400 text-sm mb-4">
                A máquina virtual Android está rodando. A janela do QEMU aparecerá separadamente no sistema.
              </p>
              <div class="flex items-center justify-center gap-4 text-sm text-gray-500">
                <span class="flex items-center gap-1">
                  <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                  Rodando (PID: {{ vmStore.vmStatus.pid }})
                </span>
              </div>
              <p class="text-xs text-gray-600 mt-4">
                Dica: Use ADB para conectar: <code class="text-green-400">adb connect localhost:5555</code>
              </p>
            </div>
          </div>

          <!-- Placeholder when VM not running -->
          <div
            v-else
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
              <div v-if="vmStore.error" class="mb-4 p-3 bg-red-900/30 border border-red-700 rounded-lg text-red-400 text-sm">
                {{ vmStore.error }}
              </div>
              <div class="flex flex-col sm:flex-row justify-center gap-4">
                <button
                  @click="startVm"
                  :disabled="vmStore.loading"
                  class="btn btn-primary px-8 py-3 text-lg disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <span v-if="vmStore.loading">Carregando...</span>
                  <span v-else>Iniciar VM</span>
                </button>
                <button
                  @click="showSettings = true"
                  class="btn btn-secondary px-8 py-3 text-lg"
                >
                  Configurações
                </button>
              </div>
              <p class="text-xs text-gray-600 mt-6">
                A VM será iniciada com Janelas GTK nativas. Requer KVM habilitado na BIOS.
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Settings Panel -->
      <SettingsPanel
        v-model="showSettings"
        @close="closeSettings"
      />
    </div>
  </div>
</template>
