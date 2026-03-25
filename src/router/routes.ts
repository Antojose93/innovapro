import type { RouteRecordRaw } from 'vue-router'

export interface AppRouteMeta {
  title: string
  icon: string
  showInNavigation: boolean
}

declare module 'vue-router' {
  interface RouteMeta extends AppRouteMeta {}
}

export const appRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'dashboard',
    component: () => import('@/views/Dashboard.vue'),
    meta: {
      title: 'Dashboard',
      icon: 'mdi-view-dashboard',
      showInNavigation: true,
    },
  },
  {
    path: '/maestros',
    name: 'maestros',
    component: () => import('@/views/Maestros.vue'),
    meta: {
      title: 'Maestros',
      icon: 'mdi-database',
      showInNavigation: true,
    },
  },
  {
    path: '/directorio',
    name: 'directorio',
    component: () => import('@/views/Directorio.vue'),
    meta: {
      title: 'Directorio',
      icon: 'mdi-card-account-details-outline',
      showInNavigation: true,
    },
  },
  {
    path: '/inventario',
    name: 'inventario',
    component: () => import('@/views/Inventario.vue'),
    meta: {
      title: 'Inventario',
      icon: 'mdi-package-variant',
      showInNavigation: true,
    },
  },
  {
    path: '/produccion',
    name: 'produccion',
    redirect: '/produccion/ordenes',
    component: () => import('@/views/produccion/ProduccionView.vue'),
    meta: {
      title: 'Produccion',
      icon: 'mdi-factory',
      showInNavigation: true,
    },
    children: [
      {
        path: 'ordenes',
        name: 'produccion-ordenes',
        component: () => import('@/views/produccion/OrdenesView.vue'),
        meta: {
          title: 'Ordenes',
          icon: 'mdi-clipboard-text-outline',
          showInNavigation: false,
        },
      },
      {
        path: 'materiales',
        name: 'produccion-materiales',
        component: () => import('@/views/produccion/MaterialesView.vue'),
        meta: {
          title: 'Materiales',
          icon: 'mdi-format-list-bulleted-square',
          showInNavigation: false,
        },
      },
    ],
  },
  {
    path: '/finanzas',
    name: 'finanzas',
    component: () => import('@/views/Finanzas.vue'),
    meta: {
      title: 'Finanzas',
      icon: 'mdi-cash-multiple',
      showInNavigation: true,
    },
  },
]
