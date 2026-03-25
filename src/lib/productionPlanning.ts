import { resolveConversionFactor } from '@/lib/unitConversion'
import type { MasterDataSnapshot } from '@/types/masterData'
import type { ProductionOrderMaterialLine, ProductionOrderSummary } from '@/types/production'

export interface PlannedOrderResult {
  recipeId: number
  plannedUnitId: number
  materials: ProductionOrderMaterialLine[]
  summary: ProductionOrderSummary
}

const round = (value: number) => Math.round(value * 10000) / 10000
const roundCurrency = (value: number) => Math.round(value * 100) / 100

export const buildProductionPlan = (
  snapshot: MasterDataSnapshot,
  productId: number,
  plannedQuantity: number
): PlannedOrderResult => {
  const product = snapshot.products.find((entry) => entry.id === productId)
  if (!product) {
    throw new Error('El producto seleccionado no existe.')
  }

  if (!product.recipeId) {
    throw new Error('El producto no tiene una receta asociada.')
  }

  const recipe = snapshot.recipes.find((entry) => entry.id === product.recipeId)
  if (!recipe) {
    throw new Error('La receta asociada al producto no existe.')
  }

  if (!recipe.outputQuantity || recipe.outputQuantity <= 0) {
    throw new Error('La receta no tiene un rendimiento valido.')
  }

  const plannedInRecipeUnit =
    plannedQuantity * resolveConversionFactor(product.unitId, recipe.outputUnitId, snapshot.conversions)
  const scaleFactor = plannedInRecipeUnit / recipe.outputQuantity

  const materials = recipe.items.map<ProductionOrderMaterialLine>((item) => {
    const material = snapshot.materials.find((entry) => entry.id === item.materialId)
    if (!material) {
      throw new Error('La receta contiene una materia prima inexistente.')
    }

    const requiredQuantity = item.quantity * scaleFactor
    const conversionFactorToStock = resolveConversionFactor(item.unitId, material.stockUnitId, snapshot.conversions)
    const stockRequiredQuantity = requiredQuantity * conversionFactorToStock

    let purchaseSuggestedQuantity: number | null = null
    try {
      const factorToPurchase = resolveConversionFactor(material.stockUnitId, material.purchaseUnitId, snapshot.conversions)
      purchaseSuggestedQuantity = round(stockRequiredQuantity * factorToPurchase)
    } catch {
      purchaseSuggestedQuantity = null
    }

    let estimatedUnitCost: number | null = null
    let estimatedLineCost: number | null = null

    if (material.currentCost !== null && material.currentCostUnitId) {
      const factorCostToStock = resolveConversionFactor(
        material.currentCostUnitId,
        material.stockUnitId,
        snapshot.conversions
      )
      estimatedUnitCost = roundCurrency(material.currentCost / factorCostToStock)
      estimatedLineCost = roundCurrency(stockRequiredQuantity * estimatedUnitCost)
    }

    return {
      materialId: material.id,
      materialCode: material.code,
      materialName: material.name,
      recipeQuantity: round(item.quantity),
      recipeUnitId: item.unitId,
      requiredQuantity: round(requiredQuantity),
      requiredUnitId: item.unitId,
      stockRequiredQuantity: round(stockRequiredQuantity),
      stockUnitId: material.stockUnitId,
      purchaseSuggestedQuantity,
      purchaseUnitId: purchaseSuggestedQuantity === null ? null : material.purchaseUnitId,
      estimatedUnitCost,
      estimatedCostUnitId: material.stockUnitId,
      estimatedLineCost,
      conversionFactorToStock: round(conversionFactorToStock),
    }
  })

  return {
    recipeId: recipe.id,
    plannedUnitId: product.unitId,
    materials,
    summary: {
      totalMaterials: materials.length,
      totalEstimatedCost: roundCurrency(
        materials.reduce((sum, line) => sum + (line.estimatedLineCost ?? 0), 0)
      ),
      missingCostLines: materials.filter((line) => line.estimatedLineCost === null).length,
    },
  }
}
