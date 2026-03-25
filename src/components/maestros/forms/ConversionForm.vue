<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { UnitConversion } from '@/types/masterData'

const props = defineProps<{
  conversionToEdit?: UnitConversion | null
}>()

const emit = defineEmits(['success'])

const masterDataStore = useMasterDataStore()
const { units } = storeToRefs(masterDataStore)

const formRef = ref<any>(null)
const formError = ref<string | null>(null)
const isEditMode = computed(() => Boolean(props.conversionToEdit))

const unitOptions = computed(() =>
  units.value.map((unit) => ({
    title: `${unit.name} (${unit.symbol})`,
    value: unit.id,
  }))
)

const formData = reactive({
  id: null as number | null,
  fromUnitId: null as number | null,
  toUnitId: null as number | null,
  factor: null as number | null,
})

watch(
  () => props.conversionToEdit,
  (conversion) => {
    if (conversion) {
      formData.id = conversion.id
      formData.fromUnitId = conversion.fromUnitId
      formData.toUnitId = conversion.toUnitId
      formData.factor = conversion.factor
    } else {
      formData.id = null
      formData.fromUnitId = null
      formData.toUnitId = null
      formData.factor = null
    }
    formError.value = null
  },
  { immediate: true }
)

const validateForm = () => {
  if (!formData.fromUnitId) return 'Selecciona la unidad de origen.'
  if (!formData.toUnitId) return 'Selecciona la unidad de destino.'
  if (formData.fromUnitId === formData.toUnitId) return 'La unidad de origen y destino no pueden ser la misma.'
  if (!formData.factor || formData.factor <= 0) return 'El factor debe ser mayor que cero.'
  return null
}

const submit = async () => {
  formError.value = validateForm()

  if (formError.value) {
    return false
  }

  try {
    if (isEditMode.value && formData.id) {
      await masterDataStore.updateUnitConversion({
        id: formData.id,
        fromUnitId: Number(formData.fromUnitId),
        toUnitId: Number(formData.toUnitId),
        factor: Number(formData.factor),
      })
    } else {
      await masterDataStore.createUnitConversion({
        fromUnitId: Number(formData.fromUnitId),
        toUnitId: Number(formData.toUnitId),
        factor: Number(formData.factor),
      })
    }

    emit('success')
    return true
  } catch (error) {
    console.error('Error al guardar conversion', error)
    return false
  }
}

defineExpose({ submit })
</script>

<template>
  <v-form ref="formRef" @submit.prevent>
    <div class="d-grid ga-4">
      <p class="text-body-2 text-medium-emphasis mb-2">
        Define el factor de multiplicacion para convertir de una unidad a otra.
      </p>

      <v-alert v-if="formError" type="warning" variant="tonal" class="mb-4">
        {{ formError }}
      </v-alert>

      <v-select v-model="formData.fromUnitId" :items="unitOptions" label="Desde (Origen)" variant="outlined" density="comfortable" />

      <div class="d-flex justify-center my-n2">
        <v-icon color="primary" opacity="0.5">mdi-arrow-down</v-icon>
      </div>

      <v-select v-model="formData.toUnitId" :items="unitOptions" label="Hacia (Destino)" variant="outlined" density="comfortable" />

      <v-text-field
        v-model.number="formData.factor"
        label="Factor de conversion"
        type="number"
        min="0"
        step="0.0001"
        hint="Ej: para pasar de Kg a gramos, el factor es 1000"
        persistent-hint
        variant="outlined"
        density="comfortable"
      />
    </div>
  </v-form>
</template>
