<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'

const masterDataStore = useMasterDataStore()
const { loading, materials, products, recipes, units } = storeToRefs(masterDataStore)

onMounted(async () => {
  if (!masterDataStore.hasData) {
    await masterDataStore.hydrate()
  }
})

const recipesWithStats = computed(() =>
  recipes.value.map((recipe) => ({
    ...recipe,
    ingredients: recipe.items.length,
    totalQuantity: recipe.items.reduce((sum, item) => sum + item.quantity, 0),
  }))
)

const productsWithoutRecipe = computed(() => products.value.filter((product) => !product.recipeId))

const resolveStatusLabel = (status: string) => status === 'active' ? 'Activa' : status === 'development' ? 'En revision' : 'Inactiva'
const resolveMaterialName = (materialId: number) => materials.value.find((material) => material.id === materialId)?.name ?? `#${materialId}`
const resolveMaterialUnit = (materialId: number) => {
  const material = materials.value.find((entry) => entry.id === materialId)
  if (!material) return ''
  return units.value.find((unit) => unit.id === material.stockUnitId)?.symbol ?? `#${material.stockUnitId}`
}
const resolveProductName = (productId: number | null) => products.value.find((product) => product.id === productId)?.name ?? 'Sin producto'
</script>

<template>
  <section class="production-tab">
    <div class="mb-4">
      <h2 class="text-h5 mb-2">Recetas</h2>
      <p class="text-body-1 text-medium-emphasis">
        Cada receta es ahora una entidad propia y puede asociarse a un producto terminado.
      </p>
    </div>

    <v-row>
      <v-col cols="12" lg="7">
        <v-card variant="outlined" rounded="lg">
          <v-card-title>Recetas activas y en desarrollo</v-card-title>
          <v-progress-linear v-if="loading" indeterminate />
          <v-table v-else>
            <thead>
              <tr>
                <th>Receta</th>
                <th>Version</th>
                <th>Estado</th>
                <th>Producto</th>
                <th>Insumos</th>
                <th>Cantidad total</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="recipe in recipesWithStats" :key="recipe.id">
                <td class="font-weight-medium">{{ recipe.name }}</td>
                <td>{{ recipe.version }}</td>
                <td>{{ resolveStatusLabel(recipe.status) }}</td>
                <td>{{ resolveProductName(recipe.productId) }}</td>
                <td>{{ recipe.ingredients }}</td>
                <td>{{ recipe.totalQuantity }}</td>
              </tr>
              <tr v-if="recipesWithStats.length === 0">
                <td colspan="6" class="text-center py-6 text-medium-emphasis">
                  No hay recetas registradas.
                </td>
              </tr>
            </tbody>
          </v-table>
        </v-card>
      </v-col>

      <v-col cols="12" lg="5">
        <v-card variant="outlined" rounded="lg" class="mb-4">
          <v-card-title>Seguimiento</v-card-title>
          <v-list>
            <v-list-item prepend-icon="mdi-notebook-outline" title="Recetas disponibles" :subtitle="`${recipesWithStats.length} configurada(s)`" />
            <v-list-item prepend-icon="mdi-alert-circle-outline" title="Productos sin receta" :subtitle="`${productsWithoutRecipe.length} pendiente(s)`" />
            <v-list-item prepend-icon="mdi-flask-empty-outline" title="Materias primas activas" :subtitle="`${materials.length} en catalogo`" />
          </v-list>
        </v-card>

        <v-card variant="outlined" rounded="lg">
          <v-card-title>Detalle rapido</v-card-title>
          <v-expansion-panels variant="accordion">
            <v-expansion-panel v-for="recipe in recipesWithStats" :key="recipe.id">
              <v-expansion-panel-title>
                {{ recipe.name }}
              </v-expansion-panel-title>
              <v-expansion-panel-text>
                <div v-if="recipe.items.length === 0" class="text-medium-emphasis">
                  Sin insumos cargados.
                </div>
                <v-list v-else density="compact">
                  <v-list-item
                    v-for="item in recipe.items"
                    :key="`${recipe.id}-${item.materialId}`"
                    :title="resolveMaterialName(item.materialId)"
                    :subtitle="`${item.quantity} ${resolveMaterialUnit(item.materialId)}`"
                  />
                </v-list>
              </v-expansion-panel-text>
            </v-expansion-panel>
          </v-expansion-panels>
        </v-card>
      </v-col>
    </v-row>
  </section>
</template>
