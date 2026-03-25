<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { Material } from '@/types/masterData'

const props = defineProps<{
  materialToEdit?: Material | null
}>()

const emit = defineEmits(['success', 'cancel'])

const masterDataStore = useMasterDataStore()
// Traemos costHistory y selectedMaterialId para gestionar los costos
const { units, costHistory, saving, selectedMaterialId } = storeToRefs(masterDataStore)

const form = ref<any>(null)
const materialFormError = ref<string | null>(null)
const costFormError = ref<string | null>(null)

const isEditMode = computed(() => !!props.materialToEdit)

const unitOptions = computed(() =>
  units.value.map((unit) => ({
    title: `${unit.name} (${unit.symbol})`,
    value: unit.id,
  }))
)

// --- ESTADO DEL MATERIAL ---
const materialForm = reactive({
  id: null as number | null,
  code: '',
  name: '',
  stockUnitId: null as number | null,
  purchaseUnitId: null as number | null,
  initialCost: null as number | null,
  initialCostUnitId: null as number | null,
})

// --- ESTADO DEL COSTO ---
const costForm = reactive({
  unitId: null as number | null,
  unitCost: null as number | null,
})

// Cargar datos al abrir el diálogo
watch(
  () => props.materialToEdit,
  async (material) => {
    if (material) {
      // Cargar info base
      materialForm.id = material.id
      materialForm.code = material.code
      materialForm.name = material.name
      materialForm.stockUnitId = material.stockUnitId
      materialForm.purchaseUnitId = material.purchaseUnitId
      materialForm.initialCost = null
      materialForm.initialCostUnitId = material.purchaseUnitId

      // Lógica de costos
      selectedMaterialId.value = material.id
      costForm.unitId = material.purchaseUnitId
      await masterDataStore.refreshCostHistory(material.id)
    } else {
      // Reset si es nuevo
      materialForm.id = null
      materialForm.code = ''
      materialForm.name = ''
      materialForm.stockUnitId = null
      materialForm.purchaseUnitId = null
      materialForm.initialCost = null
      materialForm.initialCostUnitId = null
      
      selectedMaterialId.value = null
      costForm.unitId = null
      costForm.unitCost = null
    }
    materialFormError.value = null
    costFormError.value = null
  },
  { immediate: true }
)

// --- VALIDACIONES Y SUBMITS ---
const validateMaterialForm = () => {
  if (!materialForm.code.trim()) return 'Debes ingresar un código para la materia prima.'
  if (!materialForm.name.trim()) return 'Debes ingresar un nombre para la materia prima.'
  if (!materialForm.stockUnitId) return 'Selecciona la unidad de inventario.'
  if (!materialForm.purchaseUnitId) return 'Selecciona la unidad de compra.'

  if (!isEditMode.value && materialForm.initialCost !== null && materialForm.initialCost < 0) {
    return 'El costo inicial no puede ser negativo.'
  }
  if (!isEditMode.value && materialForm.initialCost !== null && materialForm.initialCost !== undefined && !materialForm.initialCostUnitId) {
    return 'Selecciona la unidad correspondiente al costo inicial.'
  }
  return null
}

const validateCostForm = () => {
  if (!selectedMaterialId.value) return 'Selecciona una materia prima antes de registrar un costo.'
  if (!costForm.unitId) return 'Selecciona la unidad del costo.'
  if (costForm.unitCost === null || Number.isNaN(costForm.unitCost)) return 'Ingresa un valor de costo válido.'
  if (costForm.unitCost < 0) return 'El costo no puede ser negativo.'
  return null
}

// Guarda la información base del material (Llamado por el botón del Dialog)
const submit = async () => {
  materialFormError.value = validateMaterialForm()
  if (materialFormError.value) return false

  try {
    if (isEditMode.value && materialForm.id) {
      await masterDataStore.updateMaterial({
        id: materialForm.id,
        code: materialForm.code,
        name: materialForm.name,
        stockUnitId: Number(materialForm.stockUnitId),
        purchaseUnitId: Number(materialForm.purchaseUnitId),
      })
    } else {
      await masterDataStore.createMaterial({
        code: materialForm.code,
        name: materialForm.name,
        stockUnitId: Number(materialForm.stockUnitId),
        purchaseUnitId: Number(materialForm.purchaseUnitId),
        initialCost: materialForm.initialCost,
        initialCostUnitId: materialForm.initialCostUnitId,
      })
    }
    emit('success')
    return true
  } catch (error) {
    console.error("Error al guardar material", error)
    return false
  }
}

// Registra un nuevo costo en el historial (Llamado internamente)
const submitCost = async () => {
  costFormError.value = validateCostForm()
  if (costFormError.value || !selectedMaterialId.value || !costForm.unitId || costForm.unitCost === null) return

  await masterDataStore.recordMaterialCost({
    materialId: selectedMaterialId.value,
    unitId: costForm.unitId,
    unitCost: Number(costForm.unitCost),
  })

  // Refresca el historial pero mantiene el modal abierto
  await masterDataStore.refreshCostHistory(selectedMaterialId.value)
  costForm.unitCost = null
  costFormError.value = null
}

// Helpers
const resolveUnitLabel = (unitId: number) => units.value.find((unit) => unit.id === unitId)?.symbol ?? `#${unitId}`

// Exponer submit general al padre
defineExpose({ submit })
</script>

<template>
  <v-form ref="form" @submit.prevent>
    <div class="d-grid ga-4">
      <v-alert v-if="materialFormError" type="warning" variant="tonal" class="mb-2">
        {{ materialFormError }}
      </v-alert>

      <div class="text-subtitle-2 font-weight-bold text-medium-emphasis">Datos Generales</div>
      <v-row dense>
        <v-col cols="12" md="6">
          <v-text-field v-model="materialForm.code" label="Código" variant="outlined" density="comfortable" hide-details />
        </v-col>
        <v-col cols="12" md="6">
          <v-text-field v-model="materialForm.name" label="Nombre" variant="outlined" density="comfortable" hide-details />
        </v-col>
        <v-col cols="12" md="6">
          <v-select v-model="materialForm.stockUnitId" :items="unitOptions" label="Unidad de inventario" variant="outlined" density="comfortable" hide-details />
        </v-col>
        <v-col cols="12" md="6">
          <v-select v-model="materialForm.purchaseUnitId" :items="unitOptions" label="Unidad de compra" variant="outlined" density="comfortable" hide-details />
        </v-col>
      </v-row>

      <template v-if="!isEditMode">
        <v-divider class="my-2"></v-divider>
        <div class="text-subtitle-2 font-weight-bold text-medium-emphasis">Costo Inicial (Opcional)</div>
        <v-row dense>
          <v-col cols="12" md="6">
            <v-text-field v-model.number="materialForm.initialCost" label="Valor del costo" type="number" min="0" step="0.01" variant="outlined" density="comfortable" prefix="$" hide-details />
          </v-col>
          <v-col cols="12" md="6">
            <v-select v-model="materialForm.initialCostUnitId" :items="unitOptions" label="Unidad del costo" variant="outlined" density="comfortable" hide-details />
          </v-col>
        </v-row>
      </template>

      <template v-if="isEditMode">
        <v-divider class="my-3 border-opacity-50"></v-divider>
        <div class="text-subtitle-2 font-weight-bold text-medium-emphasis d-flex align-center">
          <v-icon icon="mdi-currency-usd" class="mr-1" size="small"/> Histórico y Registro de Costos
        </div>

        <v-alert v-if="costFormError" type="warning" variant="tonal" class="mb-2" density="compact">
          {{ costFormError }}
        </v-alert>

        <v-sheet class="bg-grey-lighten-4 pa-3 rounded-lg border">
          <v-row dense align="center">
            <v-col cols="12" sm="5">
              <v-text-field v-model.number="costForm.unitCost" label="Nuevo costo" type="number" min="0" step="0.01" prefix="$" variant="outlined" density="compact" bg-color="white" hide-details />
            </v-col>
            <v-col cols="12" sm="4">
              <v-select v-model="costForm.unitId" :items="unitOptions" label="Unidad" variant="outlined" density="compact" bg-color="white" hide-details />
            </v-col>
            <v-col cols="12" sm="3" class="text-right">
              <v-btn color="secondary" variant="flat" size="small" :loading="saving" block @click="submitCost">
                Añadir Costo
              </v-btn>
            </v-col>
          </v-row>
        </v-sheet>

        <v-table density="compact" class="border rounded-lg mt-2">
          <thead class="bg-grey-lighten-5">
            <tr>
              <th class="text-caption font-weight-bold">Fecha de Registro</th>
              <th class="text-caption font-weight-bold">Unidad</th>
              <th class="text-caption font-weight-bold text-right">Costo</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="entry in costHistory" :key="entry.id">
              <td class="text-caption">{{ entry.effectiveAt }}</td>
              <td class="text-caption">{{ resolveUnitLabel(entry.unitId) }}</td>
              <td class="text-caption text-right font-weight-medium">${{ entry.unitCost.toFixed(2) }}</td>
            </tr>
            <tr v-if="costHistory.length === 0">
              <td colspan="3" class="text-center text-caption py-4 text-medium-emphasis">
                No hay historial de costos registrado.
              </td>
            </tr>
          </tbody>
        </v-table>
      </template>
    </div>
  </v-form>
</template>