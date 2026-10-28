<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  vmRunning: boolean
  vmPid?: number
  showSidebar: boolean
}

interface Emits {
  (e: 'toggle-sidebar'): void
  (e: 'toggle-settings'): void
  (e: 'start-vm'): void
  (e: 'stop-vm'): void
  (e: 'restart-vm'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const starting = ref(false)
const stopping = ref(false)

function handleStartVm() {
  starting.value = true
  emit('start-vm')
  setTimeout(() => { starting.value = false }, 2000)
}

function handleStopVm() {
  stopping.value = true
  emit('stop-vm')
  setTimeout(() => { stopping.value = false }, 2000)
}

function handleRestartVm() {
  emit('restart-vm')
}
</script>

<template>
  <header class="h-14 bg-gray-900/80 backdrop-blur-sm border-b border-gray-800 flex items-center justify-between px-4">
    <!-- Left: App title + VM status -->
    <div class="flex items-center gap-4">
      <button
        @click="$emit('toggle-sidebar')"
        class="p-2 rounded-lg text-gray-400 hover:text-white hover:bg-gray-800 transition-colors lg:hidden"
        aria-label="Toggle sidebar"
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 bg-green-600/20 rounded-lg flex items-center justify-center">
          <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
          </svg>
        </div>
        <span class="font-semibold text-white text-lg">LinuxStacks</span>
      </div>

      <!-- VM Status Badge -->
      <div class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-gray-800/50 border border-gray-700">
        <span
          class="w-2 h-2 rounded-full transition-colors"
          :class="props.vmRunning ? 'bg-green-500 animate-pulse' : 'bg-gray-500'"
        />
        <span class="text-xs font-medium text-gray-300">
          {{ props.vmRunning ? 'Rodando' : 'Parado' }}
        </span>
        <span v-if="props.vmPid" class="text-xs text-gray-500 font-mono">
          (PID: {{ props.vmPid }})
        </span>
      </div>
    </div>

    <!-- Center: VM Controls -->
    <div class="flex items-center gap-2">
      <button
        @click="handleStartVm"
        :disabled="props.vmRunning || starting"
        class="btn btn-primary flex items-center gap-2 px-4 py-2"
        :class="{ 'opacity-50 cursor-not-allowed': props.vmRunning || starting }"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span v-if="starting">Iniciando...</span>
        <span v-else>Iniciar VM</span>
      </button>

      <button
        @click="handleStopVm"
        :disabled="!props.vmRunning || stopping"
        class="btn btn-danger flex items-center gap-2 px-4 py-2"
        :class="{ 'opacity-50 cursor-not-allowed': !props.vmRunning || stopping }"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
        </svg>
        <span v-if="stopping">Parando...</span>
        <span v-else>Parar VM</span>
      </button>

      <button
        @click="handleRestartVm"
        :disabled="!props.vmRunning"
        class="btn btn-secondary flex items-center gap-2 px-4 py-2"
        :class="{ 'opacity-50 cursor-not-allowed': !props.vmRunning }"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        <span>Reiniciar</span>
      </button>
    </div>

    <!-- Right: Actions -->
    <div class="flex items-center gap-2">
      <button
        @click="$emit('toggle-settings')"
        class="btn btn-secondary p-2"
        aria-label="Configurações"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>

      <button
        @click="$emit('toggle-sidebar')"
        class="btn btn-secondary p-2"
        aria-label="Toggle sidebar"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
        </svg>
      </button>
    </div>
  </header>
</template>
