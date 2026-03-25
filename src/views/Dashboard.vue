<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { desktopService } from '@/services/desktop'
import type { SystemHealth } from '@/types/masterData'

interface Order {
  id: number
  cliente: string
  producto: string
  cantidad: number
  estado: string
  fecha: string
}

const kpis = ref([
  { title: 'Ventas', value: '$125,450', icon: 'mdi-currency-usd', color: 'success' },
  { title: 'Ordenes activas', value: '24', icon: 'mdi-clipboard-list', color: 'primary' },
  { title: 'Stock critico', value: '8', icon: 'mdi-alert-circle', color: 'warning' },
  { title: 'CxC', value: '$45,200', icon: 'mdi-account-cash', color: 'info' },
])

const headers = [
  { title: 'ID', key: 'id' },
  { title: 'Cliente', key: 'cliente' },
  { title: 'Producto', key: 'producto' },
  { title: 'Cantidad', key: 'cantidad' },
  { title: 'Estado', key: 'estado' },
  { title: 'Fecha', key: 'fecha' },
]

const orders = ref<Order[]>([
  { id: 1001, cliente: 'Empresa A', producto: 'Widget X', cantidad: 150, estado: 'En Proceso', fecha: '2026-03-20' },
  { id: 1002, cliente: 'Empresa B', producto: 'Widget Y', cantidad: 200, estado: 'Pendiente', fecha: '2026-03-19' },
  { id: 1003, cliente: 'Empresa C', producto: 'Widget Z', cantidad: 100, estado: 'Completada', fecha: '2026-03-18' },
  { id: 1004, cliente: 'Empresa D', producto: 'Widget W', cantidad: 300, estado: 'En Proceso', fecha: '2026-03-17' },
  { id: 1005, cliente: 'Empresa E', producto: 'Widget V', cantidad: 250, estado: 'Pendiente', fecha: '2026-03-16' },
])

const backendStatus = ref<SystemHealth | null>(null)

onMounted(async () => {
  backendStatus.value = await desktopService.healthCheck()
})
</script>

<template>
  <div>
    <h1 class="text-h4 mb-6">Dashboard</h1>

    <v-row class="mb-6">
      <v-col v-for="kpi in kpis" :key="kpi.title" cols="12" sm="6" md="3">
        <v-card :color="kpi.color" dark>
          <v-card-text>
            <div class="d-flex align-center justify-space-between">
              <div>
                <div class="text-subtitle-2">{{ kpi.title }}</div>
                <div class="text-h5 font-weight-bold">{{ kpi.value }}</div>
              </div>
              <v-icon size="48" class="opacity-50">{{ kpi.icon }}</v-icon>
            </div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-card>
      <v-card-text class="pb-0">
        <v-alert
          v-if="backendStatus"
          :type="backendStatus.status === 'ok' ? 'success' : 'info'"
          variant="tonal"
        >
          Backend {{ backendStatus.status }} | {{ backendStatus.appName }} |
          {{ backendStatus.environment }} | v{{ backendStatus.version }}
        </v-alert>
      </v-card-text>
      <v-card-title>
        <v-icon class="mr-2">mdi-clipboard-list</v-icon>
        Ultimas 5 ordenes
      </v-card-title>
      <v-data-table
        :headers="headers"
        :items="orders"
        :items-per-page="5"
        class="elevation-1"
      />
    </v-card>
  </div>
</template>
