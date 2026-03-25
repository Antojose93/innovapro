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

const activeProducts = computed(() => products.value.filter((product) => product.status === 'active'))
const productsWithoutRecipe = computed(() => products.value.filter((product) => !product.recipeId))
const averageIngredients = computed(() => {
  if (products.value.length === 0) return 0
  const total = products.value.reduce((sum, product) => {
    const recipe = recipes.value.find((entry) => entry.id === product.recipeId)
    return sum + (recipe?.items.length ?? 0)
  }, 0)
  return total / products.value.length
})

const resolveUnitLabel = (unitId: number) => units.value.find((unit) => unit.id === unitId)?.symbol ?? `#${unitId}`
const resolveStatusLabel = (status: string) => status === 'active' ? 'Activo' : status === 'development' ? 'En desarrollo' : 'Inactivo'
const resolveRecipe = (recipeId: number | null) => recipes.value.find((recipe) => recipe.id === recipeId) ?? null
const resolveMaterialName = (materialId: number) => materials.value.find((material) => material.id === materialId)?.name ?? `#${materialId}`
</script>

<template>
  <section class="production-tab">
    <div class="mb-4">
      <h2 class="text-h5 mb-2">Productos terminados</h2>
      <p class="text-body-1 text-medium-emphasis">
        Consulta los productos con su receta asociada, unidad base y estado operativo.
      </p>
    </div>

    <v-row class="mb-4">
      <v-col cols="12" md="4">
        <v-card variant="outlined" rounded="lg">
          <v-card-text>
            <div class="text-overline text-medium-emphasis">Total productos</div>
            <div class="text-h4 font-weight-bold">{{ products.length }}</div>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="12" md="4">
        <v-card variant="outlined" rounded="lg">
          <v-card-text>
            <div class="text-overline text-medium-emphasis">Activos</div>
            <div class="text-h4 font-weight-bold">{{ activeProducts.length }}</div>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="12" md="4">
        <v-card variant="outlined" rounded="lg">
          <v-card-text>
            <div class="text-overline text-medium-emphasis">Promedio insumos por receta</div>
            <div class="text-h4 font-weight-bold">{{ averageIngredients.toFixed(1) }}</div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" lg="8">
        <v-card variant="outlined" rounded="lg">
          <v-card-title>Productos configurados</v-card-title>
          <v-progress-linear v-if="loading" indeterminate />
          <v-table v-else>
            <thead>
              <tr>
                <th>Codigo</th>
                <th>Nombre</th>
                <th>Unidad</th>
                <th>Estado</th>
                <th>Receta</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="product in products" :key="product.id">
                <td>{{ product.code }}</td>
                <td class="font-weight-medium">{{ product.name }}</td>
                <td>{{ resolveUnitLabel(product.unitId) }}</td>
                <td>{{ resolveStatusLabel(product.status) }}</td>
                <td>{{ resolveRecipe(product.recipeId)?.items.length ?? 0 }} insumo(s)</td>
              </tr>
              <tr v-if="products.length === 0">
                <td colspan="5" class="text-center py-6 text-medium-emphasis">
                  No hay productos registrados.
                </td>
              </tr>
            </tbody>
          </v-table>
        </v-card>
      </v-col>

      <v-col cols="12" lg="4">
        <v-card variant="outlined" rounded="lg" class="mb-4">
          <v-card-title>Control de recetas</v-card-title>
          <v-list>
            <v-list-item title="Productos sin receta" :subtitle="`${productsWithoutRecipe.length} pendiente(s)`" prepend-icon="mdi-alert-outline" />
            <v-list-item title="Materias primas disponibles" :subtitle="`${materials.length} registradas`" prepend-icon="mdi-flask-outline" />
            <v-list-item title="Productos activos" :subtitle="`${activeProducts.length} listos para producir`" prepend-icon="mdi-check-circle-outline" />
          </v-list>
        </v-card>

        <v-card variant="outlined" rounded="lg">
          <v-card-title>Ultimas recetas asociadas</v-card-title>
          <v-list lines="three">
            <v-list-item
              v-for="product in products.slice(0, 4)"
              :key="product.id"
              :title="product.name"
              :subtitle="resolveRecipe(product.recipeId)?.items.length ? resolveRecipe(product.recipeId)?.items.map((item) => `${resolveMaterialName(item.materialId)} x ${item.quantity}`).join(' | ') : 'Sin receta tecnica'"
            />
            <v-list-item v-if="products.length === 0" title="Sin informacion" subtitle="Crea productos y recetas desde Maestros para verlos aqui." />
          </v-list>
        </v-card>
      </v-col>
    </v-row>
  </section>
</template>
