<script setup lang="ts">
import { ref, computed } from 'vue'
import { useVmStore } from '@/stores/vmStore'

interface Props {
  modelValue: boolean
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'open-settings'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()
const vmStore = useVmStore()

const localOpen = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const collapsed = ref(false)

const menuItems = [
  { id: 'vm', label: 'Máquina Virtual', icon: 'monitor' },
  { id: 'apps', label: 'Aplicativos', icon: 'grid' },
  { id: 'files', label: 'Arquivos', icon: 'folder' },
  { id: 'settings', label: 'Configurações', icon: 'settings' },
  { id: 'about', label: 'Sobre', icon: 'info' },
] as const

function navigate(id: string) {
  if (id === 'settings') {
    emit('open-settings')
  }
}
</script>

<template>
  <Transition name="slide">
    <aside
      :class="[
        'fixed left-0 top-0 h-screen bg-gray-900 border-r border-gray-800 z-20 transition-all duration-300',
        collapsed ? 'w-16' : 'w-70'
      ]"
      @mouseenter="collapsed = false"
      @mouseleave="collapsed = true"
    >
      <!-- Header -->
      <div class="flex items-center justify-between h-16 px-4 border-b border-gray-800">
        <div class="flex items-center gap-3" v-show="!collapsed">
          <div class="w-8 h-8 bg-green-600/20 rounded-lg flex items-center justify-center">
            <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
            </svg>
          </div>
          <span class="font-semibold text-white text-lg">LinuxStacks</span>
        </div>

        <button
          @click="collapsed = !collapsed"
          class="p-1.5 rounded-lg text-gray-400 hover:text-white hover:bg-gray-800 transition-colors"
          :aria-label="collapsed ? 'Expandir' : 'Colapsar'"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              :d="collapsed ? 'M9 5l7 7-7 7' : 'M15 19l-7-7 7-7'" />
          </svg>
        </button>
      </div>

      <!-- Navigation -->
      <nav class="flex-1 p-3 space-y-1 overflow-y-auto scrollbar-thin" v-show="!collapsed">
        <div class="px-2 py-1 text-xs font-semibold text-gray-500 uppercase tracking-wider">
          Navegação
        </div>

        <button
          v-for="item in menuItems"
          :key="item.id"
          @click="navigate(item.id)"
          class="sidebar-item w-full"
        >
          <svg class="w-5 h-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path v-if="item.icon === 'monitor'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
            <path v-else-if="item.icon === 'grid'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
            <path v-else-if="item.icon === 'folder'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-1-1H5a2 2 0 00-2 2v8a2 2 0 002 2h14a2 2 0 002-2V7a2 2 0 00-2-2h-6a2 2 0 01-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2h14a2 2 0 002-2V7a2 2 0 00-2-2h-6a2 2 0 01-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2h14a2 2 0 002-2V7a2 2 0 00-2-2h-6a2 2 0 01-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2h14a2 2 0 002-2V7a2 2 0 00-2-2h-6a2 2 0 01-2-2H5" />
            <path v-else-if="item.icon === 'settings'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span>{{ item.label }}</span>
        </button>
      </nav>

      <!-- Collapsed icons -->
      <nav class="flex flex-col items-center space-y-2 p-2" v-show="collapsed">
        <button
          v-for="item in menuItems"
          :key="item.id"
          @click="navigate(item.id)"
          class="p-2 rounded-lg text-gray-400 hover:text-white hover:bg-gray-800 transition-colors"
          :title="item.label"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path v-if="item.icon === 'monitor'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
            <path v-else-if="item.icon === 'grid'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
            <path v-else-if="item.icon === 'folder'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-1-1H5a2 2 0 00-2 2v8a2 2 0 002 2h14a2 2 0 002-2V7a2 2 0 00-2-2h-6a2 2 0 01-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2h14a2 2 0 002-2V7a2 2 0 00-2-2h-6a2 2 0 01-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2h14a2 2 0 002-2V7a2 2 0 00-2-2h-6a2 2 0 01-2-2H5" />
            <path v-else-if="item.icon === 'settings'" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </nav>

      <!-- Footer -->
      <div class="p-3 border-t border-gray-800" v-show="!collapsed">
        <div class="flex items-center gap-3 text-xs text-gray-500">
          <span class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full" :class="vmStore.vmStatus.running ? 'bg-green-500' : 'bg-gray-500'" />
            <span>{{ vmStore.vmStatus.running ? 'Rodando' : 'Parado' }}</span>
          </span>
        </div>
        <div class="mt-2 text-xs text-gray-500">
          LinuxStacks v1.0.0
        </div>
      </div>
    </aside>
  </Transition>

  <!-- Overlay for mobile -->
  <Transition name="fade">
    <div
      v-if="localOpen"
      class="fixed inset-0 bg-black/50 z-40 lg:hidden"
      @click="emit('update:modelValue', false)"
    />
  </Transition>
</template>

<style scoped>
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.3s ease;
}
.slide-enter-from,
.slide-leave-to {
  transform: translateX(-100%);
}
</style>
