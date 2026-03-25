<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { desktopService } from '@/services/desktop'
import type { ProductionTab } from '@/types/masterData'

const productionTabs = ref<ProductionTab[]>([])

onMounted(async () => {
  productionTabs.value = await desktopService.listProductionTabs()
})
</script>

<template>
  <div class="production-page">
    <div class="mb-6">
      <h1 class="text-h4 mb-2">Produccion</h1>
      <p class="text-subtitle-1 text-medium-emphasis">
        Administra las opciones principales del modulo de produccion desde una sola vista.
      </p>
    </div>

    <v-card>
      <v-tabs color="primary" align-tabs="start" grow>
        <v-tab
          v-for="tab in productionTabs"
          :key="tab.route"
          :prepend-icon="tab.icon"
          :to="tab.route"
          rounded="lg"
        >
          {{ tab.label }}
        </v-tab>
      </v-tabs>

      <v-divider />

      <v-card-text class="pa-6">
        <router-view />
      </v-card-text>
    </v-card>
  </div>
</template>

<style scoped>
.production-page {
  display: grid;
  gap: 24px;
}
</style>
