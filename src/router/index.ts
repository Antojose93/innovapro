import { createRouter, createWebHistory } from 'vue-router'
import { appRoutes } from '@/router/routes'

const router = createRouter({
  history: createWebHistory(),
  routes: appRoutes,
})

router.afterEach((to) => {
  document.title = `ERP Produccion | ${to.meta.title}`
})

export default router
