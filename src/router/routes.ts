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
    redirect: '/produccion/unidades',
    component: () => import('@/views/produccion/ProduccionView.vue'),
    meta: {
      title: 'Produccion',
      icon: 'mdi-factory',
      showInNavigation: true,
    },
    children: [
      {
        path: 'unidades',
        name: 'produccion-unidades',
        component: () => import('@/views/produccion/UnidadesView.vue'),
        meta: {
          title: 'Unidades',
          icon: 'mdi-ruler-square',
          showInNavigation: false,
        },
      },
      {
        path: 'recetas',
        name: 'produccion-recetas',
        component: () => import('@/views/produccion/RecetasView.vue'),
        meta: {
          title: 'Recetas',
          icon: 'mdi-notebook-edit-outline',
          showInNavigation: false,
        },
      },
      {
        path: 'productos',
        name: 'produccion-productos',
        component: () => import('@/views/produccion/ProductosView.vue'),
        meta: {
          title: 'Productos',
          icon: 'mdi-package-variant-closed',
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
