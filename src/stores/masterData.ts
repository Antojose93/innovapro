import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { desktopService } from '@/services/desktop'
import type {
  ConversionResult,
  ConvertUnitsInput,
  CreateFinishedProductInput,
  CreateMaterialInput,
  CreateRecipeInput,
  CreateUnitConversionInput,
  CreateUnitInput,
  MasterDataSnapshot,
  MaterialCostHistoryEntry,
  RecordMaterialCostInput,
  UpdateRecipeInput,
  UpdateUnitConversionInput,
  UpdateUnitInput,
  UpdateFinishedProductInput,
  UpdateMaterialInput,
} from '@/types/masterData'

export const useMasterDataStore = defineStore('master-data', () => {
  const units = ref<MasterDataSnapshot['units']>([])
  const materials = ref<MasterDataSnapshot['materials']>([])
  const products = ref<MasterDataSnapshot['products']>([])
  const recipes = ref<MasterDataSnapshot['recipes']>([])
  const conversions = ref<MasterDataSnapshot['conversions']>([])
  const costHistory = ref<MaterialCostHistoryEntry[]>([])
  const loading = ref(false)
  const saving = ref(false)
  const error = ref<string | null>(null)
  const successMessage = ref<string | null>(null)
  const selectedMaterialId = ref<number | null>(null)
  const conversionResult = ref<ConversionResult | null>(null)

  const hasData = computed(
    () =>
      units.value.length > 0 ||
      materials.value.length > 0 ||
      products.value.length > 0 ||
      recipes.value.length > 0 ||
      conversions.value.length > 0
  )

  const applySnapshot = (snapshot: MasterDataSnapshot) => {
    units.value = snapshot.units
    materials.value = snapshot.materials
    products.value = snapshot.products
    recipes.value = snapshot.recipes
    conversions.value = snapshot.conversions
  }

  const extractErrorMessage = (taskError: unknown): string => {
    if (typeof taskError === 'string') {
      return taskError
    }

    if (taskError instanceof Error && taskError.message.trim()) {
      return taskError.message
    }

    if (typeof taskError === 'object' && taskError !== null) {
      const candidate = taskError as Record<string, unknown>

      const nestedKeys = ['message', 'error', 'details', 'cause']

      for (const key of nestedKeys) {
        const value = candidate[key]

        if (typeof value === 'string' && value.trim()) {
          return value
        }
      }
    }

    return 'No fue posible completar la operacion solicitada.'
  }

  const clearError = () => {
    error.value = null
  }

  const setSuccess = (message: string) => {
    successMessage.value = message
  }

  const clearSuccess = () => {
    successMessage.value = null
  }

  const withTask = async <T>(task: () => Promise<T>, mode: 'load' | 'save' = 'save') => {
    if (mode === 'load') {
      loading.value = true
    } else {
      saving.value = true
    }

    error.value = null

    try {
      return await task()
    } catch (taskError) {
      error.value = extractErrorMessage(taskError)
      throw taskError
    } finally {
      if (mode === 'load') {
        loading.value = false
      } else {
        saving.value = false
      }
    }
  }

  const refreshCostHistory = async (materialId = selectedMaterialId.value) => {
    if (!materialId) {
      costHistory.value = []
      return
    }

    selectedMaterialId.value = materialId
    costHistory.value = await desktopService.listMaterialCostHistory(materialId)
  }

  const hydrate = async () => {
    await withTask(async () => {
      const snapshot = await desktopService.getMasterDataSnapshot()
      applySnapshot(snapshot)

      if (!selectedMaterialId.value && snapshot.materials.length > 0) {
        selectedMaterialId.value = snapshot.materials[0].id
      }

      if (selectedMaterialId.value) {
        await refreshCostHistory(selectedMaterialId.value)
      }
    }, 'load')
  }

  const createUnit = async (input: CreateUnitInput) => {
    const snapshot = await withTask(() => desktopService.createUnit(input))
    applySnapshot(snapshot)
    setSuccess('Unidad creada correctamente.')
  }

  const updateUnit = async (input: UpdateUnitInput) => {
    const snapshot = await withTask(() => desktopService.updateUnit(input))
    applySnapshot(snapshot)
    setSuccess('Unidad actualizada correctamente.')
  }

  const deleteUnit = async (unitId: number) => {
    const snapshot = await withTask(() => desktopService.deleteUnit(unitId))
    applySnapshot(snapshot)
    setSuccess('Unidad eliminada correctamente.')
  }

  const createMaterial = async (input: CreateMaterialInput) => {
    const snapshot = await withTask(() => desktopService.createMaterial(input))
    applySnapshot(snapshot)

    const createdMaterial = snapshot.materials[0]
    if (createdMaterial) {
      selectedMaterialId.value = createdMaterial.id
      await refreshCostHistory(createdMaterial.id)
    }

    setSuccess('Materia prima creada correctamente.')
  }

  const updateMaterial = async (input: UpdateMaterialInput) => {
    const snapshot = await withTask(() => desktopService.updateMaterial(input))
    applySnapshot(snapshot)
    await refreshCostHistory(input.id)
    setSuccess('Materia prima actualizada correctamente.')
  }

  const deleteMaterial = async (materialId: number) => {
    const snapshot = await withTask(() => desktopService.deleteMaterial(materialId))
    applySnapshot(snapshot)

    if (selectedMaterialId.value === materialId) {
      selectedMaterialId.value = snapshot.materials[0]?.id ?? null
      await refreshCostHistory(selectedMaterialId.value)
    }

    setSuccess('Materia prima eliminada correctamente.')
  }

  const recordMaterialCost = async (input: RecordMaterialCostInput) => {
    costHistory.value = await withTask(() => desktopService.recordMaterialCost(input))
    await hydrate()
    setSuccess('Costo registrado correctamente.')
  }

  const createFinishedProduct = async (input: CreateFinishedProductInput) => {
    const snapshot = await withTask(() => desktopService.createFinishedProduct(input))
    applySnapshot(snapshot)
    setSuccess('Producto creado correctamente.')
  }

  const updateFinishedProduct = async (input: UpdateFinishedProductInput) => {
    const snapshot = await withTask(() => desktopService.updateFinishedProduct(input))
    applySnapshot(snapshot)
    setSuccess('Producto actualizado correctamente.')
  }

  const deleteFinishedProduct = async (productId: number) => {
    const snapshot = await withTask(() => desktopService.deleteFinishedProduct(productId))
    applySnapshot(snapshot)
    setSuccess('Producto eliminado correctamente.')
  }

  const createUnitConversion = async (input: CreateUnitConversionInput) => {
    const snapshot = await withTask(() => desktopService.createUnitConversion(input))
    applySnapshot(snapshot)
    setSuccess('Conversion guardada correctamente.')
  }

  const updateUnitConversion = async (input: UpdateUnitConversionInput) => {
    const snapshot = await withTask(() => desktopService.updateUnitConversion(input))
    applySnapshot(snapshot)
    setSuccess('Conversion actualizada correctamente.')
  }

  const deleteUnitConversion = async (conversionId: number) => {
    const snapshot = await withTask(() => desktopService.deleteUnitConversion(conversionId))
    applySnapshot(snapshot)
    setSuccess('Conversion eliminada correctamente.')
  }

  const createRecipe = async (input: CreateRecipeInput) => {
    const snapshot = await withTask(() => desktopService.createRecipe(input))
    applySnapshot(snapshot)
    setSuccess('Receta creada correctamente.')
  }

  const updateRecipe = async (input: UpdateRecipeInput) => {
    const snapshot = await withTask(() => desktopService.updateRecipe(input))
    applySnapshot(snapshot)
    setSuccess('Receta actualizada correctamente.')
  }

  const deleteRecipe = async (recipeId: number) => {
    const snapshot = await withTask(() => desktopService.deleteRecipe(recipeId))
    applySnapshot(snapshot)
    setSuccess('Receta eliminada correctamente.')
  }

  const runConversion = async (input: ConvertUnitsInput) => {
    conversionResult.value = await withTask(() => desktopService.convertUnits(input))
    setSuccess('Conversion calculada correctamente.')
  }

  return {
    units,
    materials,
    products,
    recipes,
    conversions,
    costHistory,
    loading,
    saving,
    error,
    successMessage,
    selectedMaterialId,
    conversionResult,
    hasData,
    hydrate,
    clearError,
    clearSuccess,
    refreshCostHistory,
    createUnit,
    updateUnit,
    deleteUnit,
    createMaterial,
    updateMaterial,
    deleteMaterial,
    recordMaterialCost,
    createFinishedProduct,
    updateFinishedProduct,
    deleteFinishedProduct,
    createRecipe,
    updateRecipe,
    deleteRecipe,
    createUnitConversion,
    updateUnitConversion,
    deleteUnitConversion,
    runConversion,
  }
})
