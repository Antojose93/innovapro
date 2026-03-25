<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'

import DirectoryCatalog from '@/components/directorio/DirectoryCatalog.vue'
import DirectoryEntryForm from '@/components/directorio/forms/DirectoryEntryForm.vue'
import { useDirectoryStore } from '@/stores/directory'
import type { DirectoryEntry, DirectoryEntityType } from '@/types/directory'

type DirectoryTab = 'client' | 'supplier'

const directoryStore = useDirectoryStore()
const { clients, error, loading, saving, successMessage, suppliers } = storeToRefs(directoryStore)

const currentTab = ref<DirectoryTab>('client')
const isDialogOpen = ref(false)
const formRef = ref<any>(null)
const itemToEdit = ref<DirectoryEntry | null>(null)

const tabs: Array<{
  value: DirectoryTab
  label: string
  singularLabel: string
  icon: string
}> = [
  { value: 'client', label: 'Clientes', singularLabel: 'cliente', icon: 'mdi-account-group-outline' },
  { value: 'supplier', label: 'Proveedores', singularLabel: 'proveedor', icon: 'mdi-truck-delivery-outline' },
]

const activeTab = computed(() => tabs.find((tab) => tab.value === currentTab.value) ?? tabs[0])

const summaryCards = computed(() => [
  {
    title: 'Clientes activos',
    icon: 'mdi-account-check-outline',
    value: clients.value.filter((entry) => entry.status === 'active').length,
  },
  {
    title: 'Proveedores activos',
    icon: 'mdi-store-check-outline',
    value: suppliers.value.filter((entry) => entry.status === 'active').length,
  },
  {
    title: 'Prospectos por desarrollar',
    icon: 'mdi-flag-outline',
    value: [...clients.value, ...suppliers.value].filter((entry) => entry.status === 'prospect').length,
  },
])

const showSuccess = computed({
  get: () => Boolean(successMessage.value),
  set: (value: boolean) => {
    if (!value) directoryStore.clearSuccess()
  },
})

const handleAddAction = () => {
  itemToEdit.value = null
  isDialogOpen.value = true
}

const handleEditAction = (item: DirectoryEntry) => {
  itemToEdit.value = item
  currentTab.value = item.type
  isDialogOpen.value = true
}

const handleSaveAction = async () => {
  if (!formRef.value) return
  const success = await formRef.value.submit()
  if (success) {
    isDialogOpen.value = false
  }
}

onMounted(async () => {
  if (!directoryStore.hasData) {
    await directoryStore.hydrate()
  }
})
</script>

<template>
  <div class="page-wrapper">
    <v-container fluid class="pa-0">
      <v-row class="mb-6" align="center" no-gutters>
        <v-col>
          <div class="d-flex align-center ga-3">
            <v-icon :icon="activeTab.icon" size="large" color="primary" />
            <h1 class="text-h4 font-weight-bold">Directorio</h1>
          </div>
          <p class="text-subtitle-1 text-medium-emphasis mt-1">
            Base comercial y operativa de clientes y proveedores con criterios flexibles de registro.
          </p>
        </v-col>
        <v-col cols="auto">
          <v-btn icon="mdi-refresh" variant="tonal" color="primary" :loading="loading" @click="directoryStore.hydrate()" />
        </v-col>
      </v-row>

      <v-row class="mb-6" dense>
        <v-col v-for="card in summaryCards" :key="card.title" cols="12" md="4">
          <v-card border flat rounded="lg" class="pa-4">
            <div class="d-flex align-center justify-space-between">
              <div>
                <div class="text-overline text-medium-emphasis">{{ card.title }}</div>
                <div class="text-h4 font-weight-bold">{{ card.value }}</div>
              </div>
              <v-avatar color="primary" variant="tonal" size="48">
                <v-icon :icon="card.icon" />
              </v-avatar>
            </div>
          </v-card>
        </v-col>
      </v-row>

      <v-expand-transition>
        <v-alert
          v-if="error"
          type="error"
          variant="tonal"
          closable
          class="mb-6"
          border="start"
          @click:close="directoryStore.clearError()"
        >
          {{ error }}
        </v-alert>
      </v-expand-transition>

      <v-card border flat rounded="lg">
        <v-tabs v-model="currentTab" color="primary" grow :stacked="$vuetify.display.xs">
          <v-tab v-for="tab in tabs" :key="tab.value" :value="tab.value">
            <v-icon :icon="tab.icon" :start="!$vuetify.display.xs" />
            <span class="d-none d-sm-flex">{{ tab.label }}</span>
          </v-tab>
        </v-tabs>

        <v-divider />

        <v-window v-model="currentTab" touchless>
          <v-window-item v-for="tab in tabs" :key="tab.value" :value="tab.value">
            <v-card-text class="pa-4 pa-md-6">
              <DirectoryCatalog :type="tab.value as DirectoryEntityType" @edit="handleEditAction" />
            </v-card-text>
          </v-window-item>
        </v-window>
      </v-card>
    </v-container>

    <v-fab icon="mdi-plus" color="primary" location="bottom end" size="large" app appear elevation="8" @click="handleAddAction" />

    <v-dialog v-model="isDialogOpen" max-width="820" scrollable transition="dialog-bottom-transition">
      <v-card rounded="xl">
        <v-card-title class="pa-4 d-flex align-center">
          <v-icon :icon="activeTab.icon" class="mr-3" color="primary" />
          <span class="text-h6 font-weight-bold">
            {{ itemToEdit ? 'Editar' : 'Nuevo' }} {{ activeTab.singularLabel }}
          </span>
          <v-spacer />
          <v-btn icon="mdi-close" variant="text" @click="isDialogOpen = false" />
        </v-card-title>

        <v-divider />

        <v-card-text class="pa-6">
          <DirectoryEntryForm ref="formRef" :type="currentTab" :entry-to-edit="itemToEdit" @success="isDialogOpen = false" />
        </v-card-text>

        <v-divider />

        <v-card-actions class="pa-4 bg-grey-lighten-4">
          <v-btn variant="text" rounded="lg" color="grey-darken-1" @click="isDialogOpen = false">
            Cancelar
          </v-btn>
          <v-spacer />
          <v-btn color="primary" variant="elevated" rounded="lg" :loading="saving" prepend-icon="mdi-check" @click="handleSaveAction">
            Guardar {{ activeTab.singularLabel }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-snackbar v-model="showSuccess" color="success" location="top right" variant="flat" elevation="12" rounded="lg">
      <div class="d-flex align-center ga-2">
        <v-icon icon="mdi-check-circle" />
        <span class="font-weight-medium">{{ successMessage }}</span>
      </div>
    </v-snackbar>
  </div>
</template>

<style scoped>
.page-wrapper {
  width: 100%;
}

.v-window-item {
  transition: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}
</style>
