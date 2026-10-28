<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useVmStore } from '@/stores/vmStore'

interface Props {
  modelValue: boolean
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'close'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()
const vmStore = useVmStore()

const localOpen = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const activeTab = ref<'vm' | 'android' | 'ui' | 'features' | 'about'>('vm')
const resizing = ref(false)
const downloadProgress = ref(0)
const downloading = ref(false)
const downloadStatus = ref('')
const installInstructions = ref('')const showingInstructions = ref(false)

const config = computed(() => vmStore.config)

const features = [
  { key: 'f_droid', label: 'F-Droid', description: 'Loja de apps open source' },
  { key: 'aurora_store', label: 'Aurora Store', description: 'Cliente não-oficial da Play Store' },
  { key: 'adb_integration', label: 'ADB Integration', description: 'Integração com ADB para debug' },
  { key: 'clipboard_sync', label: 'Sync Clipboard', description: 'Sincronizar clipboard host↔guest' },
  { key: 'file_share', label: 'File Sharing', description: 'Compartilhamento de arquivos host↔guest' },
] as const

type FeatureKey = typeof features[number]['key']

function close() {
  emit('close')
}

async function saveAndClose() {
  try {
    await vmStore.saveConfig()
    emit('update:modelValue', false)
  } catch (error) {
    console.error('Failed to save config:', error)
  }
}

async function resizeDisk() {
  resizing.value = true
  try {
    await vmStore.resizeDisk(32)
  } catch (error) {
    console.error('Failed to resize disk:', error)
  } finally {
    resizing.value = false
  }
}

async function checkPrerequisites() {
  try {
    const check = await vmStore.checkPrerequisites()
    if (check && !check.all_ok) {
      showingInstructions.value = true
      const instructions = await vmStore.installDependencies()
      installInstructions.value = instructions
    }
  } catch (error) {
    console.error('Prerequisite check failed:', error)
  }
}

async function downloadAndroidImage() {
  downloading.value = true
  downloadStatus.value = 'Inicializando download...'
  downloadProgress.value = 0

  try {
    const result = await vmStore.downloadAndroidImage(
      config.value.android.image_url,
      `${config.value.android.shared_folder}/android.img`
    )
    downloadStatus.value = `Download concluído: ${result.size_bytes} bytes`
    downloadProgress.value = 100
  } catch (error) {
    downloadStatus.value = `Erro no download: ${error}`
    downloadProgress.value = 0
  } finally {
    downloading.value = false
  }
}

// Auto-load config when panel opens
watch(() => props.modelValue, (val) => {
  if (val) {
    vmStore.loadConfig()
  }
})
</script>

<template>
  <Transition name="fade">
    <div v-if="localOpen" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="close" />

      <div class="relative w-full max-w-4xl max-h-[90vh] bg-gray-900 rounded-xl border border-gray-700 overflow-hidden flex flex-col">
        <!-- Header -->
        <div class="flex items-center justify-between px-6 py-4 border-b border-gray-700 flex-shrink-0">
          <h2 class="text-xl font-semibold text-white">Configurações</h2>
          <button @click="close" class="text-gray-400 hover:text-white transition-colors" aria-label="Fechar">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <!-- Tabs -->
        <div class="border-b border-gray-700 flex-shrink-0">
          <nav class="flex -mb-px px-6" aria-label="Tabs">
            <button
              v-for="tab in [{id:'vm',label:'VM'},{id:'android',label:'Android'},{id:'ui',label:'Interface'},{id:'features',label:'Features'},{id:'about',label:'Sobre'}]"
              :key="tab.id"
              :class="[
                'px-4 py-3 text-sm font-medium border-b-2 transition-colors',
                activeTab === tab.id
                  ? 'border-green-500 text-green-400'
                  : 'border-transparent text-gray-400 hover:text-white hover:border-gray-600'
              ]"
              @click="activeTab = tab.id as typeof activeTab"
            >
              {{ tab.label }}
            </button>
          </nav>
        </div>

        <!-- Content -->
        <div class="p-6 overflow-y-auto flex-1">
          <!-- VM Tab -->
          <div v-show="activeTab === 'vm'" class="space-y-6 animate-fade-in">
            <h3 class="text-lg font-medium text-white">Configurações da VM</h3>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">Memória (MB)</label>
                <input
                  type="number"
                  v-model.number="config.vm.memory_mb"
                  class="input"
                  min="1024"
                  max="32768"
                  step="512"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">CPUs</label>
                <input
                  type="number"
                  v-model.number="config.vm.cpus"
                  class="input"
                  min="1"
                  max="64"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">GPU</label>
                <select v-model="config.vm.gpu" class="input">
                  <option value="virtio-gpu">VirtIO GPU (Recomendado)</option>
                  <option value="virtio-gpu-gl">VirtIO GPU + OpenGL</option>
                  <option value="none">Sem GPU</option>
                </select>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">Rede</label>
                <select v-model="config.vm.network" class="input">
                  <option value="user-mode">User Mode (NAT)</option>
                  <option value="bridge">Bridge</option>
                  <option value="none">Sem rede</option>
                </select>
              </div>

              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-300 mb-2">Disco (caminho)</label>
                <input
                  v-model="config.vm.disk_path"
                  class="input font-mono text-sm"
                  readonly
                />
                <p class="text-xs text-gray-500 mt-1">Use o botão abaixo para redimensionar o disco</p>
              </div>

              <div class="md:col-span-2">
                <button
                  @click="resizeDisk"
                  class="btn btn-secondary"
                  :disabled="resizing"
                >
                  {{ resizing ? 'Redimensionando...' : 'Redimensionar Disco (32GB)' }}
                </button>
              </div>
            </div>

            <!-- Prerequisites -->
            <div class="card mt-4">
              <h4 class="font-medium text-white mb-3">Pré-requisitos do Sistema</h4>
              <button @click="checkPrerequisites" class="btn btn-primary" :disabled="!vmStore.vmStatus.running">
                Verificar Pré-requisitos
              </button>
              <div v-if="showingInstructions" class="mt-3 p-3 bg-gray-800 rounded-lg">
                <p class="text-sm text-gray-300 whitespace-pre-wrap">{{ installInstructions }}</p>
              </div>
            </div>
          </div>

          <!-- Android Tab -->
          <div v-show="activeTab === 'android'" class="space-y-6 animate-fade-in">
            <h3 class="text-lg font-medium text-white">Configurações do Android</h3>

            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">URL da Imagem Android</label>
                <input
                  v-model="config.android.image_url"
                  class="input font-mono text-sm"
                  placeholder="https://example.com/android.img"
                />
                <p class="text-xs text-gray-500 mt-1">
                  Use uma imagem Android x86_64 compatível com KVM (formato raw ou qcow2)
                </p>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">Pasta Compartilhada</label>
                <input
                  v-model="config.android.shared_folder"
                  class="input font-mono text-sm"
                  readonly
                />
              </div>

              <div class="card">
                <h4 class="font-medium text-white mb-3">Download da Imagem</h4>
                <p class="text-sm text-gray-400 mb-3">
                  Baixe uma imagem Android x86_64 compatível com KVM para iniciar a VM.
                </p>
                <button
                  @click="downloadAndroidImage"
                  class="btn btn-primary"
                  :disabled="downloading"
                >
                  {{ downloading ? 'Baixando...' : 'Baixar Imagem Android' }}
                </button>
                <div v-if="downloading || downloadStatus" class="mt-3">
                  <div class="h-2 bg-gray-700 rounded-full overflow-hidden">
                    <div
                      class="h-full bg-green-500 transition-all duration-300"
                      :style="{ width: downloadProgress + '%' }"
                    />
                  </div>
                  <p class="text-xs text-gray-400 mt-1">{{ downloadStatus }}</p>
                </div>
              </div>
            </div>
          </div>

          <!-- UI Tab -->
          <div v-show="activeTab === 'ui'" class="space-y-6 animate-fade-in">
            <h3 class="text-lg font-medium text-white">Interface</h3>

            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">Tema</label>
                <select v-model="config.ui.theme" class="input">
                  <option value="auto">Automático (Sistema)</option>
                  <option value="dark">Escuro</option>
                  <option value="light">Claro</option>
                </select>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-300 mb-2">Idioma</label>
                <select v-model="config.ui.language" class="input">
                  <option value="pt-BR">Português (Brasil)</option>
                  <option value="en-US">English (US)</option>
                  <option value="es-ES">Español</option>
                  <option value="fr-FR">Français</option>
                </select>
              </div>

              <div class="flex items-center gap-3">
                <input
                  type="checkbox"
                  id="showToolbar"
                  v-model="config.ui.show_toolbar"
                  class="w-5 h-5 text-green-600 border-gray-600 rounded focus:ring-green-500"
                />
                <label for="showToolbar" class="text-gray-300">Mostrar barra de ferramentas</label>
              </div>
            </div>
          </div>

          <!-- Features Tab -->
          <div v-show="activeTab === 'features'" class="space-y-6 animate-fade-in">
            <h3 class="text-lg font-medium text-white">Funcionalidades</h3>

            <div class="space-y-4">
              <div
                v-for="feature in features"
                :key="feature.key"
                class="card flex items-center justify-between"
              >
                <div>
                  <h4 class="font-medium text-white">{{ feature.label }}</h4>
                  <p class="text-sm text-gray-400">{{ feature.description }}</p>
                </div>
                <input
                  type="checkbox"
                  :checked="config.features[feature.key as FeatureKey]"
                  @change="(e) => config.features[feature.key as FeatureKey] = (e.target as HTMLInputElement).checked"
                  class="w-5 h-5 text-green-600 border-gray-600 rounded focus:ring-green-500"
                />
              </div>
            </div>
          </div>

          <!-- About Tab -->
          <div v-show="activeTab === 'about'" class="space-y-6 animate-fade-in">
            <h3 class="text-lg font-medium text-white">Sobre</h3>

            <div class="card">
              <div class="flex items-center gap-4">
                <div class="w-16 h-16 bg-green-600/20 rounded-xl flex items-center justify-center">
                  <svg class="w-10 h-10 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                      d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
                  </svg>
                </div>
                <div>
                  <h4 class="text-lg font-semibold text-white">LinuxStacks</h4>
                  <p class="text-gray-400 text-sm">Versão 1.0.0</p>
                </div>
              </div>
              <div class="grid grid-cols-2 gap-4 text-sm mt-4">
                <div>
                  <span class="text-gray-500">Tauri</span>
                  <div class="font-mono text-white">2.0</div>
                </div>
                <div>
                  <span class="text-gray-500">Rust</span>
                  <div class="font-mono text-white">1.75+</div>
                </div>
                <div>
                  <span class="text-gray-500">Vue</span>
                  <div class="font-mono text-white">3.4</div>
                </div>
                <div>
                  <span class="text-gray-500">KVM/QEMU</span>
                  <div class="font-mono text-white">Latest</div>
                </div>
              </div>
              <div class="pt-4 border-t border-gray-700 mt-4">
                <p class="text-gray-400 text-sm">
                  LinuxStacks é um emulador Android nativo para Linux,
                  construído com Tauri, Rust e Vue 3. Zero telemetria, zero anúncios.
                </p>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-700 flex-shrink-0">
          <button @click="close" class="btn btn-secondary">Cancelar</button>
          <button @click="saveAndClose" class="btn btn-primary">Salvar</button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
