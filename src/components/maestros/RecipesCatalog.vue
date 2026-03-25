<script setup lang="ts">
import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { Recipe } from '@/types/masterData'

const emit = defineEmits<{
  (e: 'edit', recipe: Recipe): void
}>()

const masterDataStore = useMasterDataStore()
const { products, recipes, saving } = storeToRefs(masterDataStore)

const search = ref('')
const deleteDialog = ref(false)
const pendingDeleteRecipe = ref<Recipe | null>(null)

const filteredRecipes = computed(() => {
  const term = search.value.trim().toLowerCase()
  return recipes.value.filter((recipe) => {
    if (!term) return true
    return (
      recipe.name.toLowerCase().includes(term) ||
      recipe.code.toLowerCase().includes(term) ||
      recipe.version.toLowerCase().includes(term)
    )
  })
})

const requestDelete = (recipe: Recipe) => {
  pendingDeleteRecipe.value = recipe
  deleteDialog.value = true
}

const confirmDelete = async () => {
  if (!pendingDeleteRecipe.value) return
  await masterDataStore.deleteRecipe(pendingDeleteRecipe.value.id)
  pendingDeleteRecipe.value = null
  deleteDialog.value = false
}

const resolveProductName = (productId: number | null) => products.value.find((product) => product.id === productId)?.name ?? 'Sin producto'
const resolveStatusLabel = (status: string) => status === 'active' ? 'Activa' : status === 'development' ? 'En desarrollo' : 'Inactiva'
</script>

<template>
  <v-card class="mb-6" flat border>
    <v-card-title class="catalog-header pa-4">
      <span class="font-weight-bold">Catalogo de recetas</span>
      <v-text-field v-model="search" label="Buscar" prepend-inner-icon="mdi-magnify" density="compact" variant="outlined" hide-details class="search-field" />
    </v-card-title>

    <v-table hover>
      <thead>
        <tr>
          <th>Codigo</th>
          <th>Nombre</th>
          <th>Version</th>
          <th>Estado</th>
          <th>Producto</th>
          <th>Insumos</th>
          <th class="text-right">Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="recipe in filteredRecipes" :key="recipe.id" class="cursor-pointer" @click="emit('edit', recipe)">
          <td>{{ recipe.code }}</td>
          <td class="font-weight-medium">{{ recipe.name }}</td>
          <td>{{ recipe.version }}</td>
          <td>{{ resolveStatusLabel(recipe.status) }}</td>
          <td>{{ resolveProductName(recipe.productId) }}</td>
          <td>{{ recipe.items.length }}</td>
          <td class="text-right actions-cell">
            <v-btn size="small" variant="text" icon="mdi-pencil-outline" color="primary" @click.stop="emit('edit', recipe)" />
            <v-btn size="small" variant="text" icon="mdi-delete-outline" color="error" @click.stop="requestDelete(recipe)" />
          </td>
        </tr>
        <tr v-if="filteredRecipes.length === 0">
          <td colspan="7" class="empty-cell">No hay recetas registradas.</td>
        </tr>
      </tbody>
    </v-table>
  </v-card>

  <v-dialog v-model="deleteDialog" max-width="400">
    <v-card rounded="xl">
      <v-card-title class="text-error d-flex align-center pa-4">
        <v-icon icon="mdi-alert" class="mr-2" />
        Confirmar eliminacion
      </v-card-title>
      <v-card-text class="pa-4 pt-0">
        Se eliminara la receta <strong>{{ pendingDeleteRecipe?.name }}</strong>.
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
.catalog-header { display: flex; justify-content: space-between; gap: 16px; align-items: center; }
.search-field { max-width: 260px; }
.actions-cell { white-space: nowrap; }
.cursor-pointer { cursor: pointer; transition: background-color 0.2s; }
.cursor-pointer:hover { background-color: rgb(var(--v-theme-grey-lighten-4)); }
.empty-cell { padding: 32px 16px; text-align: center; color: rgb(var(--v-theme-on-surface)); opacity: 0.6; }
</style>
