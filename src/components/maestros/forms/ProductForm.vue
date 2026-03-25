<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { FinishedProduct } from '@/types/masterData'

const props = defineProps<{
  productToEdit?: FinishedProduct | null
}>()

const emit = defineEmits(['success'])

const masterDataStore = useMasterDataStore()
const { recipes, units } = storeToRefs(masterDataStore)

const formRef = ref<any>(null)
const formError = ref<string | null>(null)

const isEditMode = computed(() => Boolean(props.productToEdit))

const statusOptions = [
  { title: 'Activo', value: 'active' },
  { title: 'En desarrollo', value: 'development' },
  { title: 'Inactivo', value: 'inactive' },
]

const unitOptions = computed(() =>
  units.value.map((unit) => ({
    title: `${unit.name} (${unit.symbol})`,
    value: unit.id,
  }))
)

const recipeOptions = computed(() =>
  recipes.value.map((recipe) => ({
    title: `${recipe.name} (${recipe.code})`,
    value: recipe.id,
  }))
)

const form = reactive({
  id: null as number | null,
  code: '',
  name: '',
  unitId: null as number | null,
  recipeId: null as number | null,
  category: '',
  status: 'active',
  internalCode: '',
  commercialCode: '',
})

watch(
  () => props.productToEdit,
  (product) => {
    if (product) {
      form.id = product.id
      form.code = product.code
      form.name = product.name
      form.unitId = product.unitId
      form.recipeId = product.recipeId
      form.category = product.category
      form.status = product.status
      form.internalCode = product.internalCode ?? ''
      form.commercialCode = product.commercialCode ?? ''
    } else {
      form.id = null
      form.code = ''
      form.name = ''
      form.unitId = null
      form.recipeId = null
      form.category = ''
      form.status = 'active'
      form.internalCode = ''
      form.commercialCode = ''
    }

    formError.value = null
  },
  { immediate: true }
)

const selectedRecipeSummary = computed(() => {
  if (!form.recipeId) return null
  return recipes.value.find((recipe) => recipe.id === form.recipeId) ?? null
})

const validateForm = () => {
  if (!form.code.trim()) return 'Debes ingresar un codigo para el producto.'
  if (!form.name.trim()) return 'Debes ingresar un nombre para el producto.'
  if (!form.unitId) return 'Selecciona la unidad base del producto.'
  if (!form.category.trim()) return 'Debes ingresar una categoria para el producto.'
  if (!form.status) return 'Selecciona el estado del producto.'
  return null
}

const submit = async () => {
  formError.value = validateForm()
  if (formError.value) return false

  const payload = {
    code: form.code,
    name: form.name,
    unitId: Number(form.unitId),
    recipeId: form.recipeId,
    category: form.category,
    status: form.status,
    internalCode: form.internalCode || null,
    commercialCode: form.commercialCode || null,
  }

  try {
    if (isEditMode.value && form.id) {
      await masterDataStore.updateFinishedProduct({ id: form.id, ...payload })
    } else {
      await masterDataStore.createFinishedProduct(payload)
    }
    emit('success')
    return true
  } catch (error) {
    console.error('Error al guardar producto', error)
    return false
  }
}

defineExpose({ submit })
</script>

<template>
  <v-form ref="formRef" @submit.prevent>
    <div class="d-grid ga-4">
      <v-alert v-if="formError" type="warning" variant="tonal" class="mb-4">
        {{ formError }}
      </v-alert>

      <v-text-field v-model="form.code" label="Codigo" variant="outlined" density="comfortable" />
      <v-text-field v-model="form.name" label="Nombre" variant="outlined" density="comfortable" />
      <v-text-field v-model="form.category" label="Categoria" variant="outlined" density="comfortable" />

      <v-row dense>
        <v-col cols="12" md="6">
          <v-select v-model="form.status" :items="statusOptions" label="Estado" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="6">
          <v-select v-model="form.unitId" :items="unitOptions" label="Unidad base" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.internalCode" label="Codigo interno" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.commercialCode" label="Codigo comercial" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-divider class="my-2" />

      <v-select
        v-model="form.recipeId"
        :items="recipeOptions"
        label="Receta asociada"
        variant="outlined"
        density="comfortable"
        clearable
        hint="Selecciona una receta existente. Las recetas ahora se administran por separado."
        persistent-hint
      />

      <v-alert v-if="selectedRecipeSummary" type="info" variant="tonal">
        {{ selectedRecipeSummary.name }} · {{ selectedRecipeSummary.version }} · {{ selectedRecipeSummary.items.length }} insumo(s)
      </v-alert>
    </div>
  </v-form>
</template>
