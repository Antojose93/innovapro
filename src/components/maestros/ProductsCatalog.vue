<script setup lang="ts">
import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { FinishedProduct } from '@/types/masterData'

const emit = defineEmits(['edit'])

const masterDataStore = useMasterDataStore()
const { error, products, recipes, saving, units } = storeToRefs(masterDataStore)

const search = ref('')
const selectedUnitFilter = ref<number | null>(null)
const selectedStatusFilter = ref<string | null>(null)
const selectedCategoryFilter = ref<string | null>(null)

const deleteDialog = ref(false)
const pendingDeleteProduct = ref<FinishedProduct | null>(null)

const statusOptions = [
  { title: 'Activo', value: 'active' },
  { title: 'En desarrollo', value: 'development' },
  { title: 'Inactivo', value: 'inactive' },
]

const unitOptions = computed(() =>
  units.value.map((unit) => ({ title: `${unit.name} (${unit.symbol})`, value: unit.id }))
)

const categoryOptions = computed(() =>
  [...new Set(products.value.map((product) => product.category))].map((category) => ({ title: category, value: category }))
)

const filteredProducts = computed(() => {
  const term = search.value.trim().toLowerCase()
  return products.value.filter((product) => {
    const matchesSearch =
      !term ||
      product.name.toLowerCase().includes(term) ||
      product.code.toLowerCase().includes(term) ||
      (product.internalCode ?? '').toLowerCase().includes(term) ||
      (product.commercialCode ?? '').toLowerCase().includes(term)

    const matchesUnit = !selectedUnitFilter.value || product.unitId === selectedUnitFilter.value
    const matchesStatus = !selectedStatusFilter.value || product.status === selectedStatusFilter.value
    const matchesCategory = !selectedCategoryFilter.value || product.category === selectedCategoryFilter.value
    return matchesSearch && matchesUnit && matchesStatus && matchesCategory
  })
})

const requestDelete = (product: FinishedProduct) => {
  pendingDeleteProduct.value = product
  deleteDialog.value = true
}

const confirmDelete = async () => {
  if (!pendingDeleteProduct.value) return
  await masterDataStore.deleteFinishedProduct(pendingDeleteProduct.value.id)
  pendingDeleteProduct.value = null
  deleteDialog.value = false
}

const clearFilters = () => {
  search.value = ''
  selectedUnitFilter.value = null
  selectedStatusFilter.value = null
  selectedCategoryFilter.value = null
}

const resolveStatusLabel = (status: string) => statusOptions.find((item) => item.value === status)?.title ?? status
const resolveUnitLabel = (unitId: number) => units.value.find((unit) => unit.id === unitId)?.symbol ?? `#${unitId}`
const resolveRecipeLabel = (recipeId: number | null) => recipes.value.find((recipe) => recipe.id === recipeId)?.name ?? 'Sin receta'
const resolveRecipeItems = (recipeId: number | null) => recipes.value.find((recipe) => recipe.id === recipeId)?.items.length ?? 0
</script>

<template>
  <v-card class="mb-6" flat border>
    <v-card-title class="catalog-header pa-4">
      <span class="font-weight-bold">Productos fabricados</span>
      <div class="catalog-filters">
        <v-text-field v-model="search" label="Buscar" prepend-inner-icon="mdi-magnify" density="compact" variant="outlined" hide-details class="search-field" />
        <v-select v-model="selectedCategoryFilter" :items="categoryOptions" label="Categoria" density="compact" variant="outlined" hide-details clearable class="filter-field" />
        <v-select v-model="selectedStatusFilter" :items="statusOptions" label="Estado" density="compact" variant="outlined" hide-details clearable class="filter-field" />
        <v-select v-model="selectedUnitFilter" :items="unitOptions" label="Unidad" density="compact" variant="outlined" hide-details clearable class="filter-field" />
        <v-btn variant="tonal" size="small" @click="clearFilters">Limpiar</v-btn>
      </div>
    </v-card-title>

    <v-alert v-if="error" type="error" variant="tonal" class="mx-6 mb-4">
      {{ error }}
    </v-alert>

    <v-table hover>
      <thead>
        <tr>
          <th>Codigo</th>
          <th>Nombre</th>
          <th>Categoria</th>
          <th>Unidad</th>
          <th>Estado</th>
          <th>Receta</th>
          <th class="text-right">Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="product in filteredProducts" :key="product.id" class="cursor-pointer" @click="$emit('edit', product)">
          <td>{{ product.code }}</td>
          <td class="font-weight-medium">{{ product.name }}</td>
          <td><v-chip size="small" variant="tonal">{{ product.category }}</v-chip></td>
          <td>{{ resolveUnitLabel(product.unitId) }}</td>
          <td>
            <v-chip size="small" :color="product.status === 'active' ? 'success' : product.status === 'inactive' ? 'error' : 'warning'" variant="flat">
              {{ resolveStatusLabel(product.status) }}
            </v-chip>
          </td>
          <td>
            <div class="text-body-2 font-weight-medium">{{ resolveRecipeLabel(product.recipeId) }}</div>
            <div class="text-caption text-medium-emphasis">{{ resolveRecipeItems(product.recipeId) }} insumo(s)</div>
          </td>
          <td class="actions-cell text-right">
            <v-btn size="small" variant="text" icon="mdi-pencil-outline" color="primary" @click.stop="$emit('edit', product)" />
            <v-btn size="small" variant="text" color="error" icon="mdi-delete-outline" @click.stop="requestDelete(product)" />
          </td>
        </tr>
        <tr v-if="filteredProducts.length === 0">
          <td colspan="7" class="empty-cell">No hay productos que coincidan con los filtros actuales.</td>
        </tr>
      </tbody>
    </v-table>
  </v-card>

  <v-dialog v-model="deleteDialog" max-width="400">
    <v-card rounded="xl">
      <v-card-title class="text-error d-flex align-center pa-4">
        <v-icon icon="mdi-alert" class="mr-2" /> Confirmar eliminacion
      </v-card-title>
      <v-card-text class="pa-4 pt-0">
        Se eliminara el producto <strong>{{ pendingDeleteProduct?.name }}</strong>.
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
.catalog-filters { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; margin-top: 8px;}
.catalog-header { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 16px; align-items: center;}
.empty-cell { padding: 32px 16px; text-align: center; color: rgb(var(--v-theme-on-surface)); opacity: 0.6; }
.filter-field { min-width: 150px; max-width: 180px; }
.search-field { max-width: 260px; }
</style>
