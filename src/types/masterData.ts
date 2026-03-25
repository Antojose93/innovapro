export interface SystemHealth {
  appName: string
  version: string
  environment: string
  status: string
}

export interface ProductionTab {
  key: string
  label: string
  icon: string
  route: string
}

export interface Unit {
  id: number
  code: string
  name: string
  symbol: string
}

export interface Material {
  id: number
  code: string
  name: string
  stockUnitId: number
  purchaseUnitId: number
  currentCost: number | null
  currentCostUnitId: number | null
  lastCostAt: string | null
}

export interface MaterialCostHistoryEntry {
  id: number
  materialId: number
  unitId: number
  unitCost: number
  effectiveAt: string
}

export interface FinishedProduct {
  id: number
  code: string
  name: string
  unitId: number
  recipeId: number | null
  category: string
  status: string
  internalCode: string | null
  commercialCode: string | null
}

export interface RecipeItem {
  materialId: number
  quantity: number
  unitId: number
}

export interface Recipe {
  id: number
  code: string
  name: string
  productId: number | null
  status: string
  version: string
  outputQuantity: number
  outputUnitId: number
  notes: string | null
  items: RecipeItem[]
}

export interface UnitConversion {
  id: number
  fromUnitId: number
  toUnitId: number
  factor: number
}

export interface MasterDataSnapshot {
  units: Unit[]
  materials: Material[]
  products: FinishedProduct[]
  recipes: Recipe[]
  conversions: UnitConversion[]
}

export interface CreateUnitInput {
  code: string
  name: string
  symbol: string
}

export interface UpdateUnitInput {
  id: number
  code: string
  name: string
  symbol: string
}

export interface CreateMaterialInput {
  code: string
  name: string
  stockUnitId: number
  purchaseUnitId: number
  initialCost?: number | null
  initialCostUnitId?: number | null
}

export interface UpdateMaterialInput {
  id: number
  code: string
  name: string
  stockUnitId: number
  purchaseUnitId: number
}

export interface RecordMaterialCostInput {
  materialId: number
  unitId: number
  unitCost: number
}

export interface CreateFinishedProductInput {
  code: string
  name: string
  unitId: number
  recipeId?: number | null
  category: string
  status: string
  internalCode?: string | null
  commercialCode?: string | null
}

export interface UpdateFinishedProductInput {
  id: number
  code: string
  name: string
  unitId: number
  recipeId?: number | null
  category: string
  status: string
  internalCode?: string | null
  commercialCode?: string | null
}

export interface CreateRecipeInput {
  code: string
  name: string
  productId?: number | null
  status: string
  version: string
  outputQuantity: number
  outputUnitId: number
  notes?: string | null
  items: RecipeItem[]
}

export interface UpdateRecipeInput {
  id: number
  code: string
  name: string
  productId?: number | null
  status: string
  version: string
  outputQuantity: number
  outputUnitId: number
  notes?: string | null
  items: RecipeItem[]
}

export interface CreateUnitConversionInput {
  fromUnitId: number
  toUnitId: number
  factor: number
}

export interface UpdateUnitConversionInput {
  id: number
  fromUnitId: number
  toUnitId: number
  factor: number
}

export interface ConvertUnitsInput {
  fromUnitId: number
  toUnitId: number
  quantity: number
}

export interface ConversionResult {
  factor: number
  convertedQuantity: number
}
