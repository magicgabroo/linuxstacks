<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { PrerequisiteCheck } from '@/types'

const currentStep = ref(1)
const totalSteps = 5
const installing = ref(false)
const installProgress = ref(0)
const installStep = ref('')
const prereqCheck = ref<PrerequisiteCheck | null>(null)
const emit = defineEmits<{
  (e: 'complete'): void
}>()

const steps = [
  { id: 1, title: 'Início' },
  { id: 2, title: 'Verificação' },
  { id: 3, title: 'Download' },
  { id: 4, title: 'Configuração' },
  { id: 5, title: 'Concluído' },
]

const progressPercent = computed(() => Math.round((currentStep.value / totalSteps) * 100))

async function nextStep() {
  if (currentStep.value < totalSteps) {
    currentStep.value++
    await executeStep(currentStep.value)
  }
}

async function executeStep(step: number) {
  if (step === 2 && !installing.value) {
    installing.value = true
    installStep.value = 'Verificando pré-requisitos...'
    installProgress.value = 20

    try {
      const check = await invoke<Record<string, unknown>>('check_prerequisites')
      prereqCheck.value = {
        kvm: Boolean(check.kvm),
        qemu: Boolean(check.qemu),
        libvirt: Boolean(check.libvirt),
        kvm_modules: Boolean(check.kvm_modules),
        user_in_kvm_group: Boolean(check.user_in_kvm_group),
        all_ok: Boolean(check.all_ok),
      }
      if (!check.all_ok) {
        installStep.value = 'Pré-requisitos faltando - gerando instruções...'
        installProgress.value = 40
        const instructions = await invoke<string>('install_dependencies')
        installStep.value = 'Siga as instruções no painel de configurações'
        installProgress.value = 50
      } else {
        installStep.value = 'Todos os pré-requisitos atendidos!'
        installProgress.value = 50
      }
    } catch (error) {
      console.error('Prerequisite check failed:', error)
      installStep.value = 'Erro na verificação'
    } finally {
      installing.value = false
    }
  }

  if (step === 3) {
    installStep.value = 'Imagem Android pronta para download'
    installProgress.value = 70
  }

  if (step === 4) {
    installStep.value = 'Preparando disco virtual...'
    installProgress.value = 80
    try {
      await invoke('create_disk_image', { sizeGb: 32 })
      installStep.value = 'Disco configurado com sucesso!'
      installProgress.value = 95
    } catch (error) {
      console.error('Disk config failed:', error)
      installStep.value = 'Disco já existe ou erro na configuração'
      installProgress.value = 90
    }
  }

  if (step === 5) {
    installStep.value = 'LinuxStacks pronto!'
    installProgress.value = 100
  }
}

function skipWizard() {
  emit('complete')
}

function finishWizard() {
  emit('complete')
}

onMounted(() => {
  executeStep(1)
})
</script>

<template>
  <Transition name="fade">
    <div v-if="currentStep > 0" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="skipWizard" />

      <div class="relative w-full max-w-2xl max-h-[90vh] bg-gray-900 rounded-xl border border-gray-700 overflow-hidden mx-4">
        <!-- Header -->
        <div class="flex items-center justify-between px-6 py-4 border-b border-gray-700">
          <h2 class="text-xl font-semibold text-white">Assistente LinuxStacks</h2>
          <button
            @click="skipWizard"
            class="text-gray-400 hover:text-white transition-colors"
            aria-label="Fechar assistente"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <!-- Progress Steps -->
        <div class="px-6 py-4 border-b border-gray-700">
          <div class="flex items-center justify-between">
            <div v-for="(step, index) in steps" :key="step.id" class="flex items-center">
              <div class="flex items-center">
                <div
                  class="flex items-center justify-center w-8 h-8 rounded-full text-sm font-medium transition-colors"
                  :class="[
                    step.id < currentStep ? 'bg-green-600 text-white' :
                    step.id === currentStep ? 'bg-green-600 text-white ring-2 ring-green-500 ring-offset-2 ring-offset-gray-900' :
                    'bg-gray-700 text-gray-400'
                  ]"
                >
                  {{ step.id }}
                </div>
                <div
                  v-if="index < steps.length - 1"
                  class="ml-2 w-12 h-0.5"
                  :class="step.id < currentStep ? 'bg-green-600' : 'bg-gray-700'"
                />
              </div>
              <div class="ml-2 text-sm" :class="step.id <= currentStep ? 'text-green-400' : 'text-gray-500'">
                {{ step.title }}
              </div>
            </div>
          </div>

          <div class="mt-2 h-1.5 bg-gray-700 rounded-full overflow-hidden">
            <div
              class="h-full bg-green-500 transition-all duration-500 ease-out"
              :style="{ width: progressPercent + '%' }"
            />
          </div>
        </div>

        <!-- Content -->
        <div class="p-6 max-h-[50vh] overflow-y-auto">
          <div v-show="currentStep === 1" class="space-y-4 animate-fade-in">
            <h3 class="text-lg font-semibold text-white">Bem-vindo ao LinuxStacks</h3>
            <p class="text-gray-400">
              Este assistente vai configurar o LinuxStacks no seu sistema Ubuntu.
              Vamos verificar se seu sistema atende aos requisitos e preparar tudo para execução.
            </p>
            <div class="card">
              <h4 class="font-medium text-white mb-3">O que será configurado:</h4>
              <ul class="space-y-2 text-gray-300 text-sm">
                <li class="flex items-center gap-2">
                  <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                  Verificação KVM/QEMU no sistema
                </li>
                <li class="flex items-center gap-2">
                  <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                  Imagem Android x86_64 para KVM
                </li>
                <li class="flex items-center gap-2">
                  <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                  Disco virtual de 32GB (qcow2)
                </li>
                <li class="flex items-center gap-2">
                  <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                  Rede NAT com acesso à internet
                </li>
              </ul>
            </div>
            <div class="flex gap-3">
              <button @click="nextStep" class="btn btn-primary flex-1">
                Continuar
              </button>
              <button @click="skipWizard" class="btn btn-secondary">
                Pular
              </button>
            </div>
          </div>

          <div v-show="currentStep === 2" class="space-y-4 animate-fade-in">
            <h3 class="text-lg font-semibold text-white">Verificando Sistema</h3>
            <div v-if="!installing" class="card">
              <p class="text-gray-400 mb-4">Vamos verificar se seu sistema atende aos requisitos.</p>
              <button @click="nextStep" class="btn btn-primary w-full">
                Verificar Requisitos
              </button>
            </div>

            <div v-else class="space-y-4">
              <div class="card">
                <div class="flex items-center justify-between mb-2">
                  <span class="font-medium text-white">Pré-requisitos</span>
                  <span class="text-sm" :class="prereqCheck?.all_ok ? 'text-green-400' : 'text-red-400'">
                    {{ prereqCheck?.all_ok ? 'OK' : 'Faltando' }}
                  </span>
                </div>
                <div class="space-y-1 text-sm text-gray-300" v-if="prereqCheck">
                  <div class="flex justify-between">
                    <span>KVM (/dev/kvm)</span>
                    <span :class="prereqCheck.kvm ? 'text-green-400' : 'text-red-400'">{{ prereqCheck.kvm ? 'OK' : 'Faltando' }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span>QEMU</span>
                    <span :class="prereqCheck.qemu ? 'text-green-400' : 'text-red-400'">{{ prereqCheck.qemu ? 'OK' : 'Faltando' }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span>Módulos KVM</span>
                    <span :class="prereqCheck.kvm_modules ? 'text-green-400' : 'text-red-400'">{{ prereqCheck.kvm_modules ? 'OK' : 'Faltando' }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span>Usuário no grupo kvm</span>
                    <span :class="prereqCheck.user_in_kvm_group ? 'text-green-400' : 'text-red-400'">{{ prereqCheck.user_in_kvm_group ? 'OK' : 'Faltando' }}</span>
                  </div>
                </div>
              </div>

              <div v-if="installing" class="card">
                <div class="flex items-center gap-3 mb-2">
                  <div class="w-6 h-6 border-2 border-green-500 border-t-transparent rounded-full animate-spin" />
                  <span class="text-white text-sm">{{ installStep }}</span>
                </div>
                <div class="h-2 bg-gray-700 rounded-full overflow-hidden">
                  <div class="h-full bg-green-500 transition-all duration-300" :style="{ width: installProgress + '%' }" />
                </div>
              </div>

              <button
                v-if="!installing"
                @click="nextStep"
                class="btn btn-primary w-full"
              >
                Próximo: Download
              </button>
            </div>
          </div>

          <div v-show="currentStep === 3" class="space-y-4 animate-fade-in">
            <h3 class="text-lg font-semibold text-white">Download da Imagem Android</h3>
            <div class="card">
              <p class="text-gray-400 mb-4">
                A imagem Android x86_64 será baixada (~1-2 GB). Você pode fazer o download agora
                ou pular e baixar depois nas configurações.
              </p>
              <div class="h-2 bg-gray-700 rounded-full overflow-hidden">
                <div
                  class="h-full bg-green-500 transition-all duration-500"
                  :style="{ width: installProgress + '%' }"
                />
              </div>
              <p class="text-xs text-gray-400 mt-2">{{ installStep }}</p>
              <div class="flex gap-3 mt-4">
                <button @click="nextStep" class="btn btn-primary flex-1">Continuar</button>
                <button @click="skipWizard" class="btn btn-secondary">Pular Download</button>
              </div>
            </div>
          </div>

          <div v-show="currentStep === 4" class="space-y-4 animate-fade-in">
            <h3 class="text-lg font-semibold text-white">Configuração Final</h3>
            <div class="card">
              <p class="text-gray-400 mb-4">Criando imagem de disco virtual (32GB) e aplicando configurações.</p>
              <div class="h-2 bg-gray-700 rounded-full overflow-hidden">
                <div
                  class="h-full bg-green-500 transition-all duration-500"
                  :style="{ width: installProgress + '%' }"
                />
              </div>
              <p class="text-xs text-gray-400 mt-2">{{ installStep }}</p>
              <button @click="nextStep" class="btn btn-primary w-full mt-4">Continuar</button>
            </div>
          </div>

          <div v-show="currentStep === 5" class="space-y-6 animate-fade-in text-center">
            <div class="w-20 h-20 mx-auto mb-4 bg-green-600/20 rounded-full flex items-center justify-center">
              <svg class="w-10 h-10 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h3 class="text-2xl font-semibold text-white">LinuxStacks Pronto!</h3>
            <p class="text-gray-400">
              Configuração inicial concluída. Vá para Configurações para baixar a imagem Android
              e ajustar os recursos da VM antes de iniciar.
            </p>
            <div class="space-y-3 pt-4">
              <button @click="finishWizard" class="btn btn-primary w-full py-3 text-lg">
                Começar a Usar
              </button>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="px-6 py-4 border-t border-gray-700">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm text-gray-400">Passo {{ currentStep }} de {{ totalSteps }}</span>
            <span class="text-sm font-medium text-green-400">{{ progressPercent }}%</span>
          </div>
          <div class="h-2 bg-gray-700 rounded-full overflow-hidden">
            <div
              class="h-full bg-green-500 transition-all duration-500"
              :style="{ width: progressPercent + '%' }"
            />
          </div>
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
