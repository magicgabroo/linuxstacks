import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'

const routes = [
  { path: '/', component: App, meta: { title: 'LinuxStacks' } },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach((to) => {
  document.title = `LinuxStacks - ${to.meta.title as string || 'LinuxStacks'}`
})

export default router
