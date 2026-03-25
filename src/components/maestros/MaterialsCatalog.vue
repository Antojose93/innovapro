<script setup lang="ts">
import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { Material } from '@/types/masterData'

defineEmits<{
  (e: 'edit', material: Material): void
}>()

const masterDataStore = useMasterDataStore()
const { error, materials, saving, units } = storeToRefs(masterDataStore)

// Estado de Filtros y Búsqueda
const search = ref('')
const selectedUnitFilter = ref<number | null>(null)
const selectedFilterMode = ref<'all' | 'inventory' | 'purchase'>('all')
const onlyWithoutCost = ref(false)

// Estado de Eliminación
const deleteDialog = ref(false)
const pendingDeleteMaterial = ref<Material | null>(null)

const unitOptions = computed(() =>
  units.value.map((unit) => ({
    title: `${unit.name} (${unit.symbol})`,
    value: unit.id,
  }))
)

// Filtro Reactivo
const filteredMaterials = computed(() => {
  const term = search.value.trim().toLowerCase()

  return materials.value.filter((material) => {
    const matchesSearch = !term || material.name.toLowerCase().includes(term) || material.code.toLowerCase().includes(term)

    const matchesUnit = (() => {
      if (!selectedUnitFilter.value) return true
      if (selectedFilterMode.value === 'inventory') return material.stockUnitId === selectedUnitFilter.value
      if (selectedFilterMode.value === 'purchase') return material.purchaseUnitId === selectedUnitFilter.value
      return (material.stockUnitId === selectedUnitFilter.value || material.purchaseUnitId === selectedUnitFilter.value)
    })()

    const matchesCost = !onlyWithoutCost.value || material.currentCost === null

    return matchesSearch && matchesUnit && matchesCost
  })
})

const activeFiltersCount = computed(() => {
  let total = 0
  if (search.value.trim()) total += 1
  if (selectedUnitFilter.value) total += 1
  if (onlyWithoutCost.value) total += 1
  return total
})

// Lógica de Eliminación
const requestDeleteMaterial = (material: Material) => {
  pendingDeleteMaterial.value = material
  deleteDialog.value = true
}

const confirmDeleteMaterial = async () => {
  if (!pendingDeleteMaterial.value) return
  await masterDataStore.deleteMaterial(pendingDeleteMaterial.value.id)
  pendingDeleteMaterial.value = null
  deleteDialog.value = false
}

// Helpers visuales
const resolveUnitLabel = (unitId: number) => units.value.find((unit) => unit.id === unitId)?.symbol ?? `#${unitId}`
const formatCost = (cost: number | null) => (cost === null ? 'Sin costo' : `$${cost.toFixed(2)}`)

const clearFilters = () => {
  search.value = ''
  selectedUnitFilter.value = null
  selectedFilterMode.value = 'all'
  onlyWithoutCost.value = false
}
</script>

<template>
  <v-card flat border class="mb-6">
    <v-card-title class="catalog-header pa-4">
      <span class="font-weight-bold">Catálogo de materias primas</span>
      <div class="catalog-filters">
        <v-text-field
          v-model="search"
          label="Buscar"
          prepend-inner-icon="mdi-magnify"
          density="compact"
          variant="outlined"
          hide-details
          class="search-field"
        />
        <v-select
          v-model="selectedUnitFilter"
          :items="unitOptions"
          label="Filtrar por unidad"
          density="compact"
          variant="outlined"
          hide-details
          clearable
          class="unit-filter"
        />
        <v-select
          v-model="selectedFilterMode"
          :items="[{ title: 'Inventario o compra', value: 'all' }, { title: 'Solo inventario', value: 'inventory' }, { title: 'Solo compra', value: 'purchase' }]"
          label="Tipo de unidad"
          density="compact"
          variant="outlined"
          hide-details
          class="mode-filter"
        />
        <v-checkbox v-model="onlyWithoutCost" label="Solo sin costo" density="compact" hide-details class="mt-0" />
        <v-btn variant="tonal" size="small" :disabled="activeFiltersCount === 0" @click="clearFilters">
          Limpiar
        </v-btn>
      </div>
    </v-card-title>

    <div class="filter-summary">
      {{ filteredMaterials.length }} resultado(s)
      <span v-if="activeFiltersCount > 0">| {{ activeFiltersCount }} filtro(s) activo(s)</span>
    </div>

    <v-alert v-if="error" type="error" variant="tonal" class="mx-6 mb-4">
      {{ error }}
    </v-alert>

    <v-table hover>
      <thead>
        <tr>
          <th>Código</th>
          <th>Nombre</th>
          <th>Inventario</th>
          <th>Compra</th>
          <th>Costo actual</th>
          <th class="text-right">Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr 
          v-for="material in filteredMaterials" 
          :key="material.id" 
          class="cursor-pointer" 
          @click="$emit('edit', material)"
        >
          <td>{{ material.code }}</td>
          <td class="font-weight-medium">{{ material.name }}</td>
          <td>{{ resolveUnitLabel(material.stockUnitId) }}</td>
          <td>{{ resolveUnitLabel(material.purchaseUnitId) }}</td>
          <td>
            <v-chip size="small" :color="material.currentCost ? 'success' : 'warning'" variant="flat">
              {{ formatCost(material.currentCost) }}
            </v-chip>
          </td>
          <td class="actions-cell text-right">
            <v-btn size="small" variant="text" icon="mdi-pencil-outline" color="primary" @click.stop="$emit('edit', material)" />
            <v-btn size="small" variant="text" icon="mdi-delete-outline" color="error" @click.stop="requestDeleteMaterial(material)" />
          </td>
        </tr>
        <tr v-if="filteredMaterials.length === 0">
          <td colspan="6" class="empty-cell">
            No hay materias primas que coincidan con los filtros actuales.
          </td>
        </tr>
      </tbody>
    </v-table>
  </v-card>

  <v-dialog v-model="deleteDialog" max-width="400">
    <v-card rounded="xl">
      <v-card-title class="text-error d-flex align-center pa-4">
        <v-icon icon="mdi-alert" class="mr-2" />
        Confirmar eliminación
      </v-card-title>
      <v-card-text class="pa-4 pt-0">
        Se eliminará la materia prima <strong>{{ pendingDeleteMaterial?.name }}</strong> junto con su histórico de costos. Esta acción no se puede deshacer.
      </v-card-text>
      <v-card-actions class="pa-4 bg-grey-lighten-4">
        <v-spacer />
        <v-btn variant="text" @click="deleteDialog = false">Cancelar</v-btn>
        <v-btn color="error" variant="elevated" :loading="saving" @click="confirmDeleteMaterial">
          Eliminar
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.actions-cell { white-space: nowrap; }
.catalog-filters { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; margin-top: 8px;}
.catalog-header { display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: 16px; }
.filter-summary { padding: 0 24px 16px; font-size: 0.85rem; color: rgb(var(--v-theme-on-surface)); opacity: 0.7; }
.cursor-pointer { cursor: pointer; transition: background-color 0.2s; }
.cursor-pointer:hover { background-color: rgb(var(--v-theme-grey-lighten-4)); }
.empty-cell { padding: 32px 16px; text-align: center; color: rgb(var(--v-theme-on-surface)); opacity: 0.6; }
.search-field { max-width: 220px; }
.mode-filter, .unit-filter { min-width: 180px; max-width: 220px; }
</style>
