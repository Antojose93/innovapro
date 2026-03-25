export type ProductionOrderStatus = 'draft' | 'planned' | 'released' | 'completed' | 'cancelled'

export interface ProductionOrderMaterialLine {
  materialId: number
  materialCode: string
  materialName: string
  recipeQuantity: number
  recipeUnitId: number
  requiredQuantity: number
  requiredUnitId: number
  stockRequiredQuantity: number
  stockUnitId: number
  purchaseSuggestedQuantity: number | null
  purchaseUnitId: number | null
  estimatedUnitCost: number | null
  estimatedCostUnitId: number | null
  estimatedLineCost: number | null
  conversionFactorToStock: number
}

export interface ProductionOrderSummary {
  totalMaterials: number
  totalEstimatedCost: number
  missingCostLines: number
}

export interface ProductionOrder {
  id: number
  orderNumber: string
  productId: number
  recipeId: number
  plannedQuantity: number
  plannedUnitId: number
  plannedDate: string
  status: ProductionOrderStatus
  notes: string | null
  materials: ProductionOrderMaterialLine[]
  summary: ProductionOrderSummary
  createdAt: string
  updatedAt: string
}

export interface CreateProductionOrderInput {
  orderNumber?: string | null
  productId: number
  plannedQuantity: number
  plannedDate: string
  status: ProductionOrderStatus
  notes?: string | null
}

export interface UpdateProductionOrderInput extends CreateProductionOrderInput {
  id: number
}
