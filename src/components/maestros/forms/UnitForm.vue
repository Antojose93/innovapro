<script setup lang="ts">
import { reactive, ref, watch, computed } from 'vue'
import { useMasterDataStore } from '@/stores/masterData'
import type { Unit } from '@/types/masterData'

const props = defineProps<{
  unitToEdit?: Unit | null
}>()

const emit = defineEmits(['success'])

const masterDataStore = useMasterDataStore()
const form = ref<any>(null)
const formError = ref<string | null>(null)
const isEditMode = computed(() => Boolean(props.unitToEdit))

const formData = reactive({
  id: null as number | null,
  code: '',
  name: '',
  symbol: '',
})

watch(
  () => props.unitToEdit,
  (unit) => {
    if (unit) {
      formData.id = unit.id
      formData.code = unit.code
      formData.name = unit.name
      formData.symbol = unit.symbol
    } else {
      formData.id = null
      formData.code = ''
      formData.name = ''
      formData.symbol = ''
    }

    formError.value = null
  },
  { immediate: true }
)

const validateForm = () => {
  if (!formData.code.trim()) return 'El código es obligatorio.'
  if (!formData.name.trim()) return 'El nombre es obligatorio.'
  if (!formData.symbol.trim()) return 'El símbolo es obligatorio.'
  return null
}

const submit = async () => {
  formError.value = validateForm()

  if (formError.value) {
    return false
  }

  try {
    if (isEditMode.value && formData.id) {
      await masterDataStore.updateUnit({
        id: formData.id,
        code: formData.code,
        name: formData.name,
        symbol: formData.symbol,
      })
    } else {
      await masterDataStore.createUnit({
        code: formData.code,
        name: formData.name,
        symbol: formData.symbol,
      })
    }
    emit('success')
    return true
  } catch (error) {
    console.error("Error al guardar unidad", error)
    return false
  }
}

defineExpose({ submit })
</script>

<template>
  <v-form ref="form" @submit.prevent>
    <div class="d-grid ga-4">
      <v-alert v-if="formError" type="warning" variant="tonal" class="mb-4">
        {{ formError }}
      </v-alert>

      <v-text-field 
        v-model="formData.code" 
        label="Código" 
        hint="Ej. KG" 
        persistent-hint 
        variant="outlined" 
        density="comfortable" 
      />
      <v-text-field 
        v-model="formData.name" 
        label="Nombre" 
        hint="Ej. Kilogramo" 
        persistent-hint 
        variant="outlined" 
        density="comfortable" 
      />
      <v-text-field 
        v-model="formData.symbol" 
        label="Símbolo" 
        hint="Ej. kg" 
        persistent-hint 
        variant="outlined" 
        density="comfortable" 
      />
    </div>
  </v-form>
</template>
