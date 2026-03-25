<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { Recipe, RecipeItem } from '@/types/masterData'

const props = defineProps<{
  recipeToEdit?: Recipe | null
}>()

const emit = defineEmits(['success'])

const masterDataStore = useMasterDataStore()
const { conversions, materials, products, units } = storeToRefs(masterDataStore)

const formRef = ref<any>(null)
const formError = ref<string | null>(null)

const statusOptions = [
  { title: 'Activa', value: 'active' },
  { title: 'En desarrollo', value: 'development' },
  { title: 'Inactiva', value: 'inactive' },
]

const isEditMode = computed(() => Boolean(props.recipeToEdit))
const productOptions = computed(() => products.value.map((product) => ({ title: `${product.name} (${product.code})`, value: product.id })))
const materialOptions = computed(() => materials.value.map((material) => ({ title: `${material.name} (${material.code})`, value: material.id })))

const form = reactive({
  id: null as number | null,
  code: '',
  name: '',
  productId: null as number | null,
  status: 'active',
  version: 'v1',
  notes: '',
  items: [] as RecipeItem[],
})

watch(
  () => props.recipeToEdit,
  (recipe) => {
    if (recipe) {
      form.id = recipe.id
      form.code = recipe.code
      form.name = recipe.name
      form.productId = recipe.productId
      form.status = recipe.status
      form.version = recipe.version
      form.notes = recipe.notes ?? ''
      form.items = recipe.items.map((item) => ({
        materialId: item.materialId,
        quantity: item.quantity,
        unitId: item.unitId,
      }))
    } else {
      form.id = null
      form.code = ''
      form.name = ''
      form.productId = null
      form.status = 'active'
      form.version = 'v1'
      form.notes = ''
      form.items = []
    }

    formError.value = null
  },
  { immediate: true }
)

const unitMap = computed(() => new Map(units.value.map((unit) => [unit.id, unit])))

const inferUnitFamily = (unitId: number): string => {
  const unit = units.value.find((entry) => entry.id === unitId)
  if (!unit) return 'other'

  const fingerprint = `${unit.code} ${unit.name} ${unit.symbol}`.toLowerCase()

  if (/(kg|gram|g\b|kilo|caja.*5 kg|5 kg)/.test(fingerprint)) return 'mass'
  if (/(gal|litro|l\b|ml|mililitro)/.test(fingerprint)) return 'volume'
  if (/(metro|centimetro|milimetro|\bm\b|\bcm\b|\bmm\b|varilla|platina|tubo)/.test(fingerprint)) return 'length'
  if (/(und|unidad)/.test(fingerprint)) return 'count'

  return 'other'
}

const buildReachableUnits = (materialId: number) => {
  const material = materials.value.find((entry) => entry.id === materialId)
  if (!material) return []

  const graph = new Map<number, number[]>()
  for (const conversion of conversions.value) {
    graph.set(conversion.fromUnitId, [...(graph.get(conversion.fromUnitId) ?? []), conversion.toUnitId])
    graph.set(conversion.toUnitId, [...(graph.get(conversion.toUnitId) ?? []), conversion.fromUnitId])
  }

  const seeds = [material.stockUnitId, material.purchaseUnitId]
  const allowedFamilies = new Set(seeds.map((unitId) => inferUnitFamily(unitId)))
  const queue = [...new Set(seeds)]
  const visited = new Set(queue)

  while (queue.length > 0) {
    const current = queue.shift()
    if (!current) continue

    for (const neighbor of graph.get(current) ?? []) {
      if (!visited.has(neighbor)) {
        visited.add(neighbor)
        queue.push(neighbor)
      }
    }
  }

  return [...visited]
    .map((unitId) => unitMap.value.get(unitId))
    .filter((unit): unit is NonNullable<typeof unit> => Boolean(unit))
    .filter((unit) => allowedFamilies.has(inferUnitFamily(unit.id)))
    .map((unit) => ({
      title: `${unit.name} (${unit.symbol})`,
      value: unit.id,
    }))
}

const getUnitOptions = (materialId: number) => buildReachableUnits(materialId)

const syncItemUnit = (item: RecipeItem) => {
  const options = getUnitOptions(item.materialId)
  if (options.length === 0) {
    item.unitId = 0
    return
  }

  if (!options.some((option) => option.value === item.unitId)) {
    item.unitId = options[0].value
  }
}

const addItem = () => {
  const materialId = materials.value[0]?.id ?? 0
  const defaultUnitId = buildReachableUnits(materialId)[0]?.value ?? 0

  form.items.push({
    materialId,
    quantity: 1,
    unitId: defaultUnitId,
  })
}

const removeItem = (index: number) => {
  form.items.splice(index, 1)
}

const resolveConversionHint = (item: RecipeItem) => {
  const material = materials.value.find((entry) => entry.id === item.materialId)
  const unit = units.value.find((entry) => entry.id === item.unitId)
  if (!material || !unit) return ''

  const stockUnit = units.value.find((entry) => entry.id === material.stockUnitId)
  const purchaseUnit = units.value.find((entry) => entry.id === material.purchaseUnitId)

  return `Inventario: ${stockUnit?.symbol ?? '-'} · Compra: ${purchaseUnit?.symbol ?? '-'} · Receta: ${unit.symbol}`
}

const validateForm = () => {
  if (!form.code.trim()) return 'Debes ingresar un codigo para la receta.'
  if (!form.name.trim()) return 'Debes ingresar un nombre para la receta.'
  if (!form.version.trim()) return 'Debes ingresar una version.'
  if (form.items.length === 0) return 'Agrega al menos un insumo.'

  const seenMaterials = new Set<number>()
  for (const item of form.items) {
    if (!item.materialId) return 'Selecciona la materia prima de cada fila.'
    if (!item.unitId) return 'Selecciona la unidad de cada insumo.'
    if (seenMaterials.has(item.materialId)) return 'No repitas la misma materia prima dentro de la receta.'
    if (!Number.isFinite(item.quantity) || item.quantity <= 0) return 'Cada insumo debe tener una cantidad mayor que cero.'
    seenMaterials.add(item.materialId)
  }

  return null
}

const submit = async () => {
  formError.value = validateForm()
  if (formError.value) return false

  const payload = {
    code: form.code,
    name: form.name,
    productId: form.productId,
    status: form.status,
    version: form.version,
    notes: form.notes || null,
    items: form.items.map((item) => ({
      materialId: Number(item.materialId),
      quantity: Number(item.quantity),
      unitId: Number(item.unitId),
    })),
  }

  try {
    if (isEditMode.value && form.id) {
      await masterDataStore.updateRecipe({ id: form.id, ...payload })
    } else {
      await masterDataStore.createRecipe(payload)
    }
    emit('success')
    return true
  } catch (error) {
    console.error('Error al guardar receta', error)
    return false
  }
}

defineExpose({ submit })
</script>

<template>
  <v-form ref="formRef" @submit.prevent>
    <div class="d-grid ga-4">
      <v-alert v-if="formError" type="warning" variant="tonal">
        {{ formError }}
      </v-alert>

      <v-row dense>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.code" label="Codigo" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.name" label="Nombre" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.version" label="Version" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="6">
          <v-select v-model="form.status" :items="statusOptions" label="Estado" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-select v-model="form.productId" :items="productOptions" label="Producto asociado" variant="outlined" density="comfortable" clearable />
      <v-textarea v-model="form.notes" label="Notas" variant="outlined" density="comfortable" rows="2" />

      <v-divider class="my-2" />

      <div class="d-flex justify-space-between align-center ga-3 flex-wrap">
        <div>
          <div class="text-subtitle-1 font-weight-bold">Insumos de la receta</div>
          <div class="text-body-2 text-medium-emphasis">Define materia prima, cantidad y unidad de consumo.</div>
        </div>
        <v-btn variant="tonal" prepend-icon="mdi-plus" :disabled="materials.length === 0" @click="addItem">Agregar insumo</v-btn>
      </div>

      <div v-if="form.items.length === 0" class="recipe-empty-state">
        Aun no hay insumos en la receta.
      </div>

      <div v-else class="d-grid ga-3">
        <div v-for="(item, index) in form.items" :key="`${index}-${item.materialId}`" class="recipe-row bg-grey-lighten-4 pa-3 rounded-lg">
          <v-select
            v-model="item.materialId"
            :items="materialOptions"
            label="Materia prima"
            variant="outlined"
            density="compact"
            hide-details
            class="bg-white"
            @update:model-value="syncItemUnit(item)"
          />
          <v-text-field v-model.number="item.quantity" label="Cantidad" type="number" min="0.0001" step="0.0001" variant="outlined" density="compact" hide-details class="bg-white" />
          <v-select v-model="item.unitId" :items="getUnitOptions(item.materialId)" label="Unidad" variant="outlined" density="compact" hide-details class="bg-white" />
          <v-btn icon="mdi-delete-outline" variant="text" color="error" density="comfortable" @click="removeItem(index)" />
          <div class="recipe-hint text-caption text-medium-emphasis">
            {{ resolveConversionHint(item) }}
          </div>
        </div>
      </div>
    </div>
  </v-form>
</template>

<style scoped>
.recipe-empty-state {
  padding: 16px;
  border: 1px dashed rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 12px;
  text-align: center;
  color: rgb(var(--v-theme-on-surface));
  opacity: 0.7;
}
.recipe-row {
  display: grid;
  grid-template-columns: minmax(0, 1.5fr) minmax(110px, 0.7fr) minmax(140px, 0.9fr) auto;
  gap: 12px;
  align-items: center;
}
.recipe-hint {
  grid-column: 1 / -1;
}
@media (max-width: 600px) {
  .recipe-row { grid-template-columns: 1fr; }
}
</style>
