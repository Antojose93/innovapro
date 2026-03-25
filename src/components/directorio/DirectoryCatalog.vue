<script setup lang="ts">
import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'

import { useDirectoryStore } from '@/stores/directory'
import type { DirectoryEntry, DirectoryEntityType } from '@/types/directory'

const props = defineProps<{
  type: DirectoryEntityType
}>()

const emit = defineEmits<{
  edit: [entry: DirectoryEntry]
}>()

const directoryStore = useDirectoryStore()
const { clients, error, saving, suppliers } = storeToRefs(directoryStore)

const search = ref('')
const statusFilter = ref<string | null>(null)
const segmentFilter = ref<string | null>(null)

const deleteDialog = ref(false)
const pendingDelete = ref<DirectoryEntry | null>(null)

const statusOptions = [
  { title: 'Activo', value: 'active' },
  { title: 'Prospecto', value: 'prospect' },
  { title: 'Inactivo', value: 'inactive' },
  { title: 'Bloqueado', value: 'blocked' },
]

const currentEntries = computed(() => (props.type === 'client' ? clients.value : suppliers.value))

const segmentOptions = computed(() =>
  [...new Set(currentEntries.value.map((entry) => entry.segment).filter(Boolean))].map((segment) => ({
    title: segment,
    value: segment,
  }))
)

const filteredEntries = computed(() => {
  const term = search.value.trim().toLowerCase()

  return currentEntries.value.filter((entry) => {
    const matchesSearch =
      !term ||
      entry.displayName.toLowerCase().includes(term) ||
      (entry.legalName ?? '').toLowerCase().includes(term) ||
      (entry.contactName ?? '').toLowerCase().includes(term) ||
      (entry.documentNumber ?? '').toLowerCase().includes(term) ||
      (entry.email ?? '').toLowerCase().includes(term) ||
      (entry.city ?? '').toLowerCase().includes(term)

    const matchesStatus = !statusFilter.value || entry.status === statusFilter.value
    const matchesSegment = !segmentFilter.value || entry.segment === segmentFilter.value

    return matchesSearch && matchesStatus && matchesSegment
  })
})

const resolveStatusLabel = (status: string) => statusOptions.find((item) => item.value === status)?.title ?? status

const resolveStatusColor = (status: string) =>
  ({
    active: 'success',
    prospect: 'info',
    inactive: 'grey',
    blocked: 'error',
  })[status] ?? 'grey'

const clearFilters = () => {
  search.value = ''
  statusFilter.value = null
  segmentFilter.value = null
}

const requestDelete = (entry: DirectoryEntry) => {
  pendingDelete.value = entry
  deleteDialog.value = true
}

const confirmDelete = async () => {
  if (!pendingDelete.value) return
  await directoryStore.deleteEntry(pendingDelete.value.id)
  pendingDelete.value = null
  deleteDialog.value = false
}

const formatLocation = (entry: DirectoryEntry) => [entry.city, entry.country].filter(Boolean).join(', ') || 'Sin ubicacion'
const formatTags = (entry: DirectoryEntry) => entry.tags.slice(0, 2)
</script>

<template>
  <v-card class="mb-6" flat border>
    <v-card-title class="catalog-header pa-4">
      <span class="font-weight-bold">{{ props.type === 'client' ? 'Clientes' : 'Proveedores' }}</span>
      <div class="catalog-filters">
        <v-text-field v-model="search" label="Buscar" prepend-inner-icon="mdi-magnify" density="compact" variant="outlined" hide-details class="search-field" />
        <v-select v-model="segmentFilter" :items="segmentOptions" label="Segmento" density="compact" variant="outlined" hide-details clearable class="filter-field" />
        <v-select v-model="statusFilter" :items="statusOptions" label="Estado" density="compact" variant="outlined" hide-details clearable class="filter-field" />
        <v-btn variant="tonal" size="small" @click="clearFilters">Limpiar</v-btn>
      </div>
    </v-card-title>

    <v-alert v-if="error" type="error" variant="tonal" class="mx-6 mb-4">
      {{ error }}
    </v-alert>

    <v-table hover>
      <thead>
        <tr>
          <th>Nombre</th>
          <th>Segmento</th>
          <th>Contacto</th>
          <th>Ubicacion</th>
          <th>Estado</th>
          <th>Observaciones</th>
          <th class="text-right">Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="entry in filteredEntries" :key="entry.id" class="cursor-pointer" @click="emit('edit', entry)">
          <td>
            <div class="font-weight-medium">{{ entry.displayName }}</div>
            <div class="text-caption text-medium-emphasis">{{ entry.legalName || entry.documentNumber || 'Sin razon social registrada' }}</div>
          </td>
          <td><v-chip size="small" variant="tonal">{{ entry.segment }}</v-chip></td>
          <td>
            <div class="text-body-2 font-weight-medium">{{ entry.contactName || 'Sin contacto principal' }}</div>
            <div class="text-caption text-medium-emphasis">{{ entry.email || entry.phone || 'Sin canal principal' }}</div>
          </td>
          <td>{{ formatLocation(entry) }}</td>
          <td>
            <v-chip size="small" :color="resolveStatusColor(entry.status)" variant="flat">
              {{ resolveStatusLabel(entry.status) }}
            </v-chip>
          </td>
          <td>
            <div class="text-caption text-medium-emphasis">{{ entry.notes || 'Sin notas' }}</div>
            <div class="d-flex ga-1 mt-1 flex-wrap">
              <v-chip v-for="tag in formatTags(entry)" :key="tag" size="x-small" variant="outlined">{{ tag }}</v-chip>
            </div>
          </td>
          <td class="actions-cell text-right">
            <v-btn size="small" variant="text" icon="mdi-pencil-outline" color="primary" @click.stop="emit('edit', entry)" />
            <v-btn size="small" variant="text" color="error" icon="mdi-delete-outline" @click.stop="requestDelete(entry)" />
          </td>
        </tr>
        <tr v-if="filteredEntries.length === 0">
          <td colspan="7" class="empty-cell">No hay registros que coincidan con los filtros actuales.</td>
        </tr>
      </tbody>
    </v-table>
  </v-card>

  <v-dialog v-model="deleteDialog" max-width="420">
    <v-card rounded="xl">
      <v-card-title class="text-error d-flex align-center pa-4">
        <v-icon icon="mdi-alert" class="mr-2" /> Confirmar eliminacion
      </v-card-title>
      <v-card-text class="pa-4 pt-0">
        Se eliminara <strong>{{ pendingDelete?.displayName }}</strong> del directorio.
      </v-card-text>
      <v-card-actions class="pa-4 bg-grey-lighten-4">
        <v-spacer />
        <v-btn variant="text" @click="deleteDialog = false">Cancelar</v-btn>
        <v-btn color="error" variant="elevated" :loading="saving" @click="confirmDelete">Eliminar</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.actions-cell { white-space: nowrap; }
.cursor-pointer { cursor: pointer; transition: background-color 0.2s; }
.cursor-pointer:hover { background-color: rgb(var(--v-theme-grey-lighten-4)); }
.catalog-filters { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; margin-top: 8px; }
.catalog-header { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 16px; align-items: center; }
.empty-cell { padding: 32px 16px; text-align: center; color: rgb(var(--v-theme-on-surface)); opacity: 0.6; }
.filter-field { min-width: 150px; max-width: 180px; }
.search-field { max-width: 260px; }
</style>
