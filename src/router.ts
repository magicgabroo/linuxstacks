import { createRouter, createWebHistory } from 'vue-router'
import VmWindow from '@/components/VmWindow.vue'
import Settings from '@/components/SettingsPanel.vue'
import InstallWizard from '@/components/InstallWizard.vue'

const routes = [
  { path: '/', component: VmWindow, meta: { title: 'VM' } },
  { path: '/settings', component: Settings, meta: { title: 'Configurações' } },
  { path: '/wizard', component: InstallWizard, meta: { title: 'Assistente de Instalação' } },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach((to, from, next) => {
  document.title = `LinuxStacks - ${to.meta.title as string || 'LinuxStacks'}`
  next()
})

export default router