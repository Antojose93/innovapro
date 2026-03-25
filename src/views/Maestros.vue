<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'

import ConversionsCatalog from '@/components/maestros/ConversionsCatalog.vue'
import MaterialsCatalog from '@/components/maestros/MaterialsCatalog.vue'
import ProductsCatalog from '@/components/maestros/ProductsCatalog.vue'
import RecipesCatalog from '@/components/maestros/RecipesCatalog.vue'
import UnitsCatalog from '@/components/maestros/UnitsCatalog.vue'

import ConversionForm from '@/components/maestros/forms/ConversionForm.vue'
import MaterialForm from '@/components/maestros/forms/MaterialForm.vue'
import ProductForm from '@/components/maestros/forms/ProductForm.vue'
import RecipeForm from '@/components/maestros/forms/RecipeForm.vue'
import UnitForm from '@/components/maestros/forms/UnitForm.vue'

import { useMasterDataStore } from '@/stores/masterData'

type MasterTab = 'materials' | 'units' | 'recipes' | 'products' | 'conversions'

const masterDataStore = useMasterDataStore()
const { error, loading, saving, successMessage } = storeToRefs(masterDataStore)

const currentTab = ref<MasterTab>('materials')
const isDialogOpen = ref(false)
const formRef = ref<any>(null)
const itemToEdit = ref<any>(null)

const tabs: Array<{
  value: MasterTab
  label: string
  singularLabel: string
  icon: string
}> = [
  { value: 'materials', label: 'Materias primas', singularLabel: 'materia prima', icon: 'mdi-flask-outline' },
  { value: 'units', label: 'Unidades', singularLabel: 'unidad', icon: 'mdi-ruler-square' },
  { value: 'recipes', label: 'Recetas', singularLabel: 'receta', icon: 'mdi-notebook-edit-outline' },
  { value: 'products', label: 'Productos', singularLabel: 'producto', icon: 'mdi-package-variant-closed' },
  { value: 'conversions', label: 'Conversiones', singularLabel: 'conversion', icon: 'mdi-swap-horizontal' },
]

const activeTab = computed(() => tabs.find((tab) => tab.value === currentTab.value) ?? tabs[0])

const showSuccess = computed({
  get: () => Boolean(successMessage.value),
  set: (value: boolean) => {
    if (!value) masterDataStore.clearSuccess()
  },
})

const getComponent = (tab: MasterTab) =>
  ({
    materials: MaterialsCatalog,
    units: UnitsCatalog,
    recipes: RecipesCatalog,
    products: ProductsCatalog,
    conversions: ConversionsCatalog,
  })[tab]

const getFormComponent = (tab: MasterTab) =>
  ({
    materials: MaterialForm,
    units: UnitForm,
    recipes: RecipeForm,
    products: ProductForm,
    conversions: ConversionForm,
  })[tab]

const handleAddAction = () => {
  itemToEdit.value = null
  isDialogOpen.value = true
}

const handleEditAction = (item: unknown) => {
  itemToEdit.value = item
  isDialogOpen.value = true
}

const handleSaveAction = async () => {
  if (!formRef.value) return
  const success = await formRef.value.submit()
  if (success) {
    isDialogOpen.value = false
  }
}

onMounted(async () => {
  if (!masterDataStore.hasData) {
    await masterDataStore.hydrate()
  }
})
</script>

<template>
  <div class="page-wrapper">
    <v-container fluid class="pa-0">
      <v-row class="mb-6" align="center" no-gutters>
        <v-col>
          <div class="d-flex align-center ga-3">
            <v-icon :icon="activeTab.icon" size="large" color="primary" />
            <h1 class="text-h4 font-weight-bold">Maestros</h1>
          </div>
          <p class="text-subtitle-1 text-medium-emphasis mt-1">
            Gestion de {{ activeTab.label.toLowerCase() }} y base operativa.
          </p>
        </v-col>
        <v-col cols="auto">
          <v-btn icon="mdi-refresh" variant="tonal" color="primary" :loading="loading" @click="masterDataStore.hydrate()" />
        </v-col>
      </v-row>

      <v-expand-transition>
        <v-alert
          v-if="error"
          type="error"
          variant="tonal"
          closable
          class="mb-6"
          border="start"
          @click:close="masterDataStore.clearError()"
        >
          {{ error }}
        </v-alert>
      </v-expand-transition>

      <v-card border flat rounded="lg">
        <v-tabs v-model="currentTab" color="primary" grow :stacked="$vuetify.display.xs">
          <v-tab v-for="tab in tabs" :key="tab.value" :value="tab.value">
            <v-icon :icon="tab.icon" :start="!$vuetify.display.xs" />
            <span class="d-none d-sm-flex">{{ tab.label }}</span>
          </v-tab>
        </v-tabs>

        <v-divider />

        <v-window v-model="currentTab" touchless>
          <v-window-item v-for="tab in tabs" :key="tab.value" :value="tab.value">
            <v-card-text class="pa-4 pa-md-6">
              <component :is="getComponent(tab.value)" @edit="handleEditAction" />
            </v-card-text>
          </v-window-item>
        </v-window>
      </v-card>
    </v-container>

    <v-fab icon="mdi-plus" color="primary" location="bottom end" size="large" app appear elevation="8" @click="handleAddAction" />

    <v-dialog v-model="isDialogOpen" max-width="700" scrollable transition="dialog-bottom-transition">
      <v-card rounded="xl">
        <v-card-title class="pa-4 d-flex align-center">
          <v-icon :icon="activeTab.icon" class="mr-3" color="primary" />
          <span class="text-h6 font-weight-bold">
            {{ itemToEdit ? 'Editar' : 'Nuevo' }} {{ activeTab.singularLabel }}
          </span>
          <v-spacer />
          <v-btn icon="mdi-close" variant="text" @click="isDialogOpen = false" />
        </v-card-title>

        <v-divider />

        <v-card-text class="pa-6">
          <component
            :is="getFormComponent(currentTab)"
            ref="formRef"
            :unitToEdit="currentTab === 'units' ? itemToEdit : undefined"
            :materialToEdit="currentTab === 'materials' ? itemToEdit : undefined"
            :recipeToEdit="currentTab === 'recipes' ? itemToEdit : undefined"
            :productToEdit="currentTab === 'products' ? itemToEdit : undefined"
            :conversionToEdit="currentTab === 'conversions' ? itemToEdit : undefined"
            @success="isDialogOpen = false"
          />
        </v-card-text>

        <v-divider />

        <v-card-actions class="pa-4 bg-grey-lighten-4">
          <v-btn variant="text" rounded="lg" color="grey-darken-1" @click="isDialogOpen = false">
            Cancelar
          </v-btn>
          <v-spacer />
          <v-btn color="primary" variant="elevated" rounded="lg" :loading="saving" prepend-icon="mdi-check" @click="handleSaveAction">
            Guardar {{ activeTab.singularLabel }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-snackbar v-model="showSuccess" color="success" location="top right" variant="flat" elevation="12" rounded="lg">
      <div class="d-flex align-center ga-2">
        <v-icon icon="mdi-check-circle" />
        <span class="font-weight-medium">{{ successMessage }}</span>
      </div>
    </v-snackbar>
  </div>
</template>

<style scoped>
.page-wrapper {
  width: 100%;
}

.v-window-item {
  transition: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}
</style>
