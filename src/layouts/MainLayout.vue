<script setup lang="ts">
import { computed } from 'vue'
import { appRoutes } from '@/router/routes'
import { useAppStore } from '@/stores/app'

const appStore = useAppStore()

const navigationItems = computed(() =>
  appRoutes.flatMap((route) => {
    const meta = route.meta

    if (!meta?.showInNavigation) {
      return []
    }

    return [
      {
        title: meta.title,
        icon: meta.icon,
        path: route.path,
      },
    ]
  })
)
</script>

<template>
  <v-app>
    <v-app-bar color="primary" elevation="2">
      <v-btn icon="mdi-menu" variant="text" color="white" style="background-color: transparent !important;"
        elevation="0" aria-label="Alternar menú lateral" @click="appStore.toggleSidebar" />
      <v-app-bar-title class="font-weight-bold">ERP Producción</v-app-bar-title>
    </v-app-bar>

    <v-navigation-drawer v-model="appStore.isSidebarOpen" elevation="1">
      <v-list>
        <v-list-item prepend-avatar="https://cdn.vuetifyjs.com/images/logos/v.png" title="Mi Empresa S.A."
          subtitle="Panel de gestión" />
      </v-list>

      <v-divider />

      <v-list density="compact" nav>
        <v-list-item v-for="item in navigationItems" :key="item.path" :to="item.path" :append-icon="item.icon"
          :title="item.title" exact color="primary" rounded="lg" />
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <v-container fluid class="pa-4 pa-md-6">
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
/* Eliminamos .app-shell porque ahora usamos las clases pa-4 pa-md-6 de Vuetify */

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>