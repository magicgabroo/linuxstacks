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
  const title = to.meta.title as string
  document.title = title ? `LinuxStacks - ${title}` : 'LinuxStacks'
})

export default router
