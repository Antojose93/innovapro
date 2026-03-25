import { invoke } from '@tauri-apps/api/core'
import { isTauriApp } from '@/lib/tauri'
import { resolveConversionFactor } from '@/lib/unitConversion'
import type {
  ConversionResult,
  ConvertUnitsInput,
  CreateFinishedProductInput,
  CreateMaterialInput,
  CreateRecipeInput,
  CreateUnitConversionInput,
  CreateUnitInput,
  FinishedProduct,
  MasterDataSnapshot,
  Material,
  MaterialCostHistoryEntry,
  ProductionTab,
  RecordMaterialCostInput,
  Recipe,
  SystemHealth,
  UpdateUnitInput,
  UpdateFinishedProductInput,
  UpdateMaterialInput,
  UpdateRecipeInput,
  UpdateUnitConversionInput,
} from '@/types/masterData'

interface SystemHealthPayload {
  app_name: string
  version: string
  environment: string
  status: string
}

let nextUnitId = 100
let nextMaterialId = 100
let nextCostHistoryId = 100
let nextProductId = 100
let nextRecipeId = 100
let nextConversionId = 100

const mockSnapshot: MasterDataSnapshot = {
  units: [
    { id: 1, code: 'KG', name: 'Kilogramo', symbol: 'kg' },
    { id: 2, code: 'G', name: 'Gramo', symbol: 'g' },
    { id: 3, code: 'L', name: 'Litro', symbol: 'L' },
    { id: 4, code: 'ML', name: 'Mililitro', symbol: 'ml' },
    { id: 5, code: 'UND', name: 'Unidad', symbol: 'und' },
    { id: 6, code: 'GAL', name: 'Galon', symbol: 'gal' },
    { id: 7, code: 'M', name: 'Metro', symbol: 'm' },
    { id: 8, code: 'CM', name: 'Centimetro', symbol: 'cm' },
    { id: 9, code: 'MM', name: 'Milimetro', symbol: 'mm' },
    { id: 10, code: 'VAR6', name: 'Varilla (6 metros)', symbol: 'var-6m' },
    { id: 11, code: 'PLA6', name: 'Platina (6 metros)', symbol: 'plat-6m' },
    { id: 12, code: 'TUB6', name: 'Tubo (6 metros)', symbol: 'tubo-6m' },
    { id: 13, code: 'CAJ5', name: 'Caja (5 kg)', symbol: 'caja-5kg' },
  ],
  materials: [
    {
      id: 1,
      code: 'HIE-001',
      name: 'Varilla Corrugada 3/8" (9.5mm)',
      stockUnitId: 5,
      purchaseUnitId: 10,
      currentCost: 18500,
      currentCostUnitId: 10,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 2,
      code: 'HIE-002',
      name: 'Varilla Corrugada 1/2" (12.7mm)',
      stockUnitId: 5,
      purchaseUnitId: 10,
      currentCost: 32500,
      currentCostUnitId: 10,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 3,
      code: 'HIE-003',
      name: 'Varilla Cuadrada Lisa 1/2" (12mm)',
      stockUnitId: 5,
      purchaseUnitId: 10,
      currentCost: 38500,
      currentCostUnitId: 10,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 4,
      code: 'HIE-004',
      name: 'Platina de Hierro 1" x 1/8"',
      stockUnitId: 5,
      purchaseUnitId: 11,
      currentCost: 24000,
      currentCostUnitId: 11,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 5,
      code: 'HIE-005',
      name: 'Platina de Hierro 2" x 1/8"',
      stockUnitId: 5,
      purchaseUnitId: 11,
      currentCost: 48500,
      currentCostUnitId: 11,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 6,
      code: 'HIE-006',
      name: 'Tubo Cuadrado (Perfil) 1.5" x 1.5" Calibre 18',
      stockUnitId: 5,
      purchaseUnitId: 12,
      currentCost: 51500,
      currentCostUnitId: 12,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 7,
      code: 'SOL-001',
      name: 'Electrodo E6013 de 1/8" (Caja)',
      stockUnitId: 1,
      purchaseUnitId: 13,
      currentCost: 112500,
      currentCostUnitId: 13,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 8,
      code: 'SOL-002',
      name: 'Electrodo E6013 de 1/8" (Suelto)',
      stockUnitId: 1,
      purchaseUnitId: 1,
      currentCost: 24000,
      currentCostUnitId: 1,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 9,
      code: 'SOL-003',
      name: 'Electrodo E7018 de 1/8" (Alta resistencia)',
      stockUnitId: 1,
      purchaseUnitId: 13,
      currentCost: 130000,
      currentCostUnitId: 13,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 10,
      code: 'PIN-001',
      name: 'Thinner Corriente / Acrilico',
      stockUnitId: 6,
      purchaseUnitId: 6,
      currentCost: 32000,
      currentCostUnitId: 6,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 11,
      code: 'PIN-002',
      name: 'Pintura Anticorrosiva (Gris/Rojo)',
      stockUnitId: 6,
      purchaseUnitId: 6,
      currentCost: 57500,
      currentCostUnitId: 6,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 12,
      code: 'PIN-003',
      name: 'Laca / Esmalte Sintetico (Colores)',
      stockUnitId: 6,
      purchaseUnitId: 6,
      currentCost: 120000,
      currentCostUnitId: 6,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 13,
      code: 'CON-001',
      name: 'Disco de Corte para Metal 4.5"',
      stockUnitId: 5,
      purchaseUnitId: 5,
      currentCost: 5500,
      currentCostUnitId: 5,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 14,
      code: 'CON-002',
      name: 'Disco de Desbaste / Pulir 4.5"',
      stockUnitId: 5,
      purchaseUnitId: 5,
      currentCost: 7500,
      currentCostUnitId: 5,
      lastCostAt: '2026-03-24 09:00:00',
    },
    {
      id: 15,
      code: 'CON-003',
      name: 'Bisagra de Capsula o Torneada 5/8"',
      stockUnitId: 5,
      purchaseUnitId: 5,
      currentCost: 4000,
      currentCostUnitId: 5,
      lastCostAt: '2026-03-24 09:00:00',
    },
  ],
  products: [
    {
      id: 1,
      code: 'PT-001',
      name: 'Jarabe base',
      unitId: 3,
      recipeId: 1,
      category: 'Bases',
      status: 'active',
      internalCode: 'INT-JB-01',
      commercialCode: 'COM-JARABE-BASE',
    },
  ],
  recipes: [
    {
      id: 1,
      code: 'REC-PT-001',
      name: 'Receta Jarabe base',
      productId: 1,
      status: 'active',
      version: 'v1',
      outputQuantity: 1,
      outputUnitId: 3,
      notes: 'Receta inicial',
      items: [
        {
          materialId: 1,
          quantity: 2.5,
          unitId: 5,
        },
      ],
    },
  ],
  conversions: [
    {
      id: 1,
      fromUnitId: 1,
      toUnitId: 2,
      factor: 1000,
    },
    {
      id: 2,
      fromUnitId: 3,
      toUnitId: 4,
      factor: 1000,
    },
    {
      id: 3,
      fromUnitId: 6,
      toUnitId: 3,
      factor: 3.78541,
    },
    {
      id: 4,
      fromUnitId: 7,
      toUnitId: 8,
      factor: 100,
    },
    {
      id: 5,
      fromUnitId: 8,
      toUnitId: 9,
      factor: 10,
    },
    {
      id: 6,
      fromUnitId: 10,
      toUnitId: 7,
      factor: 6,
    },
    {
      id: 7,
      fromUnitId: 11,
      toUnitId: 7,
      factor: 6,
    },
    {
      id: 8,
      fromUnitId: 12,
      toUnitId: 7,
      factor: 6,
    },
    {
      id: 9,
      fromUnitId: 10,
      toUnitId: 5,
      factor: 1,
    },
    {
      id: 10,
      fromUnitId: 11,
      toUnitId: 5,
      factor: 1,
    },
    {
      id: 11,
      fromUnitId: 12,
      toUnitId: 5,
      factor: 1,
    },
    {
      id: 12,
      fromUnitId: 13,
      toUnitId: 1,
      factor: 5,
    },
  ],
}

const mockCostHistory: Record<number, MaterialCostHistoryEntry[]> = {
  1: [
    {
      id: 1,
      materialId: 1,
      unitId: 10,
      unitCost: 18500,
      effectiveAt: '2026-03-24 09:00:00',
    },
  ],
  2: [{ id: 2, materialId: 2, unitId: 10, unitCost: 32500, effectiveAt: '2026-03-24 09:00:00' }],
  3: [{ id: 3, materialId: 3, unitId: 10, unitCost: 38500, effectiveAt: '2026-03-24 09:00:00' }],
  4: [{ id: 4, materialId: 4, unitId: 11, unitCost: 24000, effectiveAt: '2026-03-24 09:00:00' }],
  5: [{ id: 5, materialId: 5, unitId: 11, unitCost: 48500, effectiveAt: '2026-03-24 09:00:00' }],
  6: [{ id: 6, materialId: 6, unitId: 12, unitCost: 51500, effectiveAt: '2026-03-24 09:00:00' }],
  7: [{ id: 7, materialId: 7, unitId: 13, unitCost: 112500, effectiveAt: '2026-03-24 09:00:00' }],
  8: [{ id: 8, materialId: 8, unitId: 1, unitCost: 24000, effectiveAt: '2026-03-24 09:00:00' }],
  9: [{ id: 9, materialId: 9, unitId: 13, unitCost: 130000, effectiveAt: '2026-03-24 09:00:00' }],
  10: [{ id: 10, materialId: 10, unitId: 6, unitCost: 32000, effectiveAt: '2026-03-24 09:00:00' }],
  11: [{ id: 11, materialId: 11, unitId: 6, unitCost: 57500, effectiveAt: '2026-03-24 09:00:00' }],
  12: [{ id: 12, materialId: 12, unitId: 6, unitCost: 120000, effectiveAt: '2026-03-24 09:00:00' }],
  13: [{ id: 13, materialId: 13, unitId: 5, unitCost: 5500, effectiveAt: '2026-03-24 09:00:00' }],
  14: [{ id: 14, materialId: 14, unitId: 5, unitCost: 7500, effectiveAt: '2026-03-24 09:00:00' }],
  15: [{ id: 15, materialId: 15, unitId: 5, unitCost: 4000, effectiveAt: '2026-03-24 09:00:00' }],
}

const buildProductionTabs = (): ProductionTab[] => [
  {
    key: 'unidades',
    label: 'Unidades',
    icon: 'mdi-ruler-square',
    route: '/produccion/unidades',
  },
  {
    key: 'recetas',
    label: 'Recetas',
    icon: 'mdi-notebook-edit-outline',
    route: '/produccion/recetas',
  },
  {
    key: 'productos',
    label: 'Productos',
    icon: 'mdi-package-variant-closed',
    route: '/produccion/productos',
  },
]

const cloneSnapshot = (): MasterDataSnapshot => ({
  units: [...mockSnapshot.units],
  materials: [...mockSnapshot.materials],
  products: [...mockSnapshot.products],
  recipes: [...mockSnapshot.recipes],
  conversions: [...mockSnapshot.conversions],
})

const desktopFallback = {
  async getMasterDataSnapshot(): Promise<MasterDataSnapshot> {
    return cloneSnapshot()
  },

  async createUnit(input: CreateUnitInput): Promise<MasterDataSnapshot> {
    mockSnapshot.units.push({
      id: nextUnitId++,
      code: input.code.trim().toUpperCase(),
      name: input.name.trim(),
      symbol: input.symbol.trim(),
    })

    return cloneSnapshot()
  },

  async updateUnit(input: UpdateUnitInput): Promise<MasterDataSnapshot> {
    const unit = mockSnapshot.units.find((item) => item.id === input.id)

    if (unit) {
      unit.code = input.code.trim().toUpperCase()
      unit.name = input.name.trim()
      unit.symbol = input.symbol.trim()
    }

    return cloneSnapshot()
  },

  async deleteUnit(unitId: number): Promise<MasterDataSnapshot> {
    mockSnapshot.units = mockSnapshot.units.filter((item) => item.id !== unitId)

    return cloneSnapshot()
  },

  async createMaterial(input: CreateMaterialInput): Promise<MasterDataSnapshot> {
    const material: Material = {
      id: nextMaterialId++,
      code: input.code.trim().toUpperCase(),
      name: input.name.trim(),
      stockUnitId: input.stockUnitId,
      purchaseUnitId: input.purchaseUnitId,
      currentCost: input.initialCost ?? null,
      currentCostUnitId: input.initialCost ? input.initialCostUnitId ?? input.purchaseUnitId : null,
      lastCostAt: input.initialCost ? new Date().toISOString() : null,
    }

    mockSnapshot.materials.unshift(material)

    if (input.initialCost !== null && input.initialCost !== undefined) {
      mockCostHistory[material.id] = [
        {
          id: nextCostHistoryId++,
          materialId: material.id,
          unitId: input.initialCostUnitId ?? input.purchaseUnitId,
          unitCost: input.initialCost,
          effectiveAt: new Date().toISOString(),
        },
      ]
    }

    return cloneSnapshot()
  },

  async listMaterialCostHistory(materialId: number): Promise<MaterialCostHistoryEntry[]> {
    return [...(mockCostHistory[materialId] ?? [])]
  },

  async recordMaterialCost(input: RecordMaterialCostInput): Promise<MaterialCostHistoryEntry[]> {
    const entry: MaterialCostHistoryEntry = {
      id: nextCostHistoryId++,
      materialId: input.materialId,
      unitId: input.unitId,
      unitCost: input.unitCost,
      effectiveAt: new Date().toISOString(),
    }

    mockCostHistory[input.materialId] = [entry, ...(mockCostHistory[input.materialId] ?? [])]

    const material = mockSnapshot.materials.find((item) => item.id === input.materialId)
    if (material) {
      material.currentCost = input.unitCost
      material.currentCostUnitId = input.unitId
      material.lastCostAt = entry.effectiveAt
    }

    return [...mockCostHistory[input.materialId]]
  },

  async updateMaterial(input: UpdateMaterialInput): Promise<MasterDataSnapshot> {
    const material = mockSnapshot.materials.find((item) => item.id === input.id)

    if (material) {
      material.code = input.code.trim().toUpperCase()
      material.name = input.name.trim()
      material.stockUnitId = input.stockUnitId
      material.purchaseUnitId = input.purchaseUnitId
    }

    return cloneSnapshot()
  },

  async deleteMaterial(materialId: number): Promise<MasterDataSnapshot> {
    mockSnapshot.materials = mockSnapshot.materials.filter((item) => item.id !== materialId)
    delete mockCostHistory[materialId]

    return cloneSnapshot()
  },

  async createFinishedProduct(input: CreateFinishedProductInput): Promise<MasterDataSnapshot> {
    const product: FinishedProduct = {
      id: nextProductId++,
      code: input.code.trim().toUpperCase(),
      name: input.name.trim(),
      unitId: input.unitId,
      recipeId: input.recipeId ?? null,
      category: input.category.trim(),
      status: input.status,
      internalCode: input.internalCode?.trim() || null,
      commercialCode: input.commercialCode?.trim() || null,
    }

    mockSnapshot.products.unshift(product)
    return cloneSnapshot()
  },

  async updateFinishedProduct(input: UpdateFinishedProductInput): Promise<MasterDataSnapshot> {
    const product = mockSnapshot.products.find((item) => item.id === input.id)

    if (product) {
      product.code = input.code.trim().toUpperCase()
      product.name = input.name.trim()
      product.unitId = input.unitId
      product.recipeId = input.recipeId ?? null
      product.category = input.category.trim()
      product.status = input.status
      product.internalCode = input.internalCode?.trim() || null
      product.commercialCode = input.commercialCode?.trim() || null
    }

    return cloneSnapshot()
  },

  async deleteFinishedProduct(productId: number): Promise<MasterDataSnapshot> {
    mockSnapshot.products = mockSnapshot.products.filter((item) => item.id !== productId)
    mockSnapshot.recipes = mockSnapshot.recipes.map((recipe) =>
      recipe.productId === productId ? { ...recipe, productId: null } : recipe
    )
    return cloneSnapshot()
  },

  async createRecipe(input: CreateRecipeInput): Promise<MasterDataSnapshot> {
    const recipe: Recipe = {
      id: nextRecipeId++,
      code: input.code.trim().toUpperCase(),
      name: input.name.trim(),
      productId: input.productId ?? null,
      status: input.status,
      version: input.version.trim(),
      outputQuantity: input.outputQuantity,
      outputUnitId: input.outputUnitId,
      notes: input.notes?.trim() || null,
      items: input.items.map((item) => ({
        materialId: item.materialId,
        quantity: item.quantity,
        unitId: item.unitId,
      })),
    }

    mockSnapshot.recipes.unshift(recipe)
    return cloneSnapshot()
  },

  async updateRecipe(input: UpdateRecipeInput): Promise<MasterDataSnapshot> {
    const recipe = mockSnapshot.recipes.find((item) => item.id === input.id)

    if (recipe) {
      recipe.code = input.code.trim().toUpperCase()
      recipe.name = input.name.trim()
      recipe.productId = input.productId ?? null
      recipe.status = input.status
      recipe.version = input.version.trim()
      recipe.outputQuantity = input.outputQuantity
      recipe.outputUnitId = input.outputUnitId
      recipe.notes = input.notes?.trim() || null
      recipe.items = input.items.map((item) => ({
        materialId: item.materialId,
        quantity: item.quantity,
        unitId: item.unitId,
      }))
    }

    return cloneSnapshot()
  },

  async deleteRecipe(recipeId: number): Promise<MasterDataSnapshot> {
    mockSnapshot.recipes = mockSnapshot.recipes.filter((item) => item.id !== recipeId)
    mockSnapshot.products = mockSnapshot.products.map((product) =>
      product.recipeId === recipeId ? { ...product, recipeId: null } : product
    )
    return cloneSnapshot()
  },

  async createUnitConversion(input: CreateUnitConversionInput): Promise<MasterDataSnapshot> {
    const existing = mockSnapshot.conversions.find(
      (item) => item.fromUnitId === input.fromUnitId && item.toUnitId === input.toUnitId
    )

    if (existing) {
      existing.factor = input.factor
    } else {
      mockSnapshot.conversions.unshift({
        id: nextConversionId++,
        fromUnitId: input.fromUnitId,
        toUnitId: input.toUnitId,
        factor: input.factor,
      })
    }

    return cloneSnapshot()
  },

  async updateUnitConversion(input: UpdateUnitConversionInput): Promise<MasterDataSnapshot> {
    const conversion = mockSnapshot.conversions.find((item) => item.id === input.id)

    if (conversion) {
      conversion.fromUnitId = input.fromUnitId
      conversion.toUnitId = input.toUnitId
      conversion.factor = input.factor
    }

    return cloneSnapshot()
  },

  async deleteUnitConversion(conversionId: number): Promise<MasterDataSnapshot> {
    mockSnapshot.conversions = mockSnapshot.conversions.filter((item) => item.id !== conversionId)
    return cloneSnapshot()
  },

  async convertUnits(input: ConvertUnitsInput): Promise<ConversionResult> {
    const factor = resolveConversionFactor(input.fromUnitId, input.toUnitId, mockSnapshot.conversions)

    return {
      factor,
      convertedQuantity: input.quantity * factor,
    }
  },
}

export const desktopService = {
  async healthCheck(): Promise<SystemHealth> {
    if (!isTauriApp()) {
      return {
        appName: 'ERP Produccion',
        version: 'web-only',
        environment: 'browser',
        status: 'detached',
      }
    }

    const payload = await invoke<SystemHealthPayload>('health_check')

    return {
      appName: payload.app_name,
      version: payload.version,
      environment: payload.environment,
      status: payload.status,
    }
  },

  async listProductionTabs(): Promise<ProductionTab[]> {
    if (!isTauriApp()) {
      return buildProductionTabs()
    }

    return invoke<ProductionTab[]>('list_tabs')
  },

  async getMasterDataSnapshot(): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.getMasterDataSnapshot()
    }

    return invoke<MasterDataSnapshot>('get_master_data_snapshot')
  },

  async createUnit(input: CreateUnitInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.createUnit(input)
    }

    return invoke<MasterDataSnapshot>('create_unit', { input })
  },

  async updateUnit(input: UpdateUnitInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.updateUnit(input)
    }

    return invoke<MasterDataSnapshot>('update_unit', { input })
  },

  async deleteUnit(unitId: number): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.deleteUnit(unitId)
    }

    return invoke<MasterDataSnapshot>('delete_unit', { unitId })
  },

  async createMaterial(input: CreateMaterialInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.createMaterial(input)
    }

    return invoke<MasterDataSnapshot>('create_material', { input })
  },

  async updateMaterial(input: UpdateMaterialInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.updateMaterial(input)
    }

    return invoke<MasterDataSnapshot>('update_material', { input })
  },

  async deleteMaterial(materialId: number): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.deleteMaterial(materialId)
    }

    return invoke<MasterDataSnapshot>('delete_material', { materialId })
  },

  async listMaterialCostHistory(materialId: number): Promise<MaterialCostHistoryEntry[]> {
    if (!isTauriApp()) {
      return desktopFallback.listMaterialCostHistory(materialId)
    }

    return invoke<MaterialCostHistoryEntry[]>('list_material_cost_history', { materialId })
  },

  async recordMaterialCost(input: RecordMaterialCostInput): Promise<MaterialCostHistoryEntry[]> {
    if (!isTauriApp()) {
      return desktopFallback.recordMaterialCost(input)
    }

    return invoke<MaterialCostHistoryEntry[]>('record_material_cost', { input })
  },

  async createFinishedProduct(input: CreateFinishedProductInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.createFinishedProduct(input)
    }

    return invoke<MasterDataSnapshot>('create_finished_product', { input })
  },

  async updateFinishedProduct(input: UpdateFinishedProductInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.updateFinishedProduct(input)
    }

    return invoke<MasterDataSnapshot>('update_finished_product', { input })
  },

  async deleteFinishedProduct(productId: number): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.deleteFinishedProduct(productId)
    }

    return invoke<MasterDataSnapshot>('delete_finished_product', { productId })
  },

  async createRecipe(input: CreateRecipeInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.createRecipe(input)
    }

    return invoke<MasterDataSnapshot>('create_recipe', { input })
  },

  async updateRecipe(input: UpdateRecipeInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.updateRecipe(input)
    }

    return invoke<MasterDataSnapshot>('update_recipe', { input })
  },

  async deleteRecipe(recipeId: number): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.deleteRecipe(recipeId)
    }

    return invoke<MasterDataSnapshot>('delete_recipe', { recipeId })
  },

  async createUnitConversion(input: CreateUnitConversionInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.createUnitConversion(input)
    }

    return invoke<MasterDataSnapshot>('create_unit_conversion', { input })
  },

  async updateUnitConversion(input: UpdateUnitConversionInput): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.updateUnitConversion(input)
    }

    return invoke<MasterDataSnapshot>('update_unit_conversion', { input })
  },

  async deleteUnitConversion(conversionId: number): Promise<MasterDataSnapshot> {
    if (!isTauriApp()) {
      return desktopFallback.deleteUnitConversion(conversionId)
    }

    return invoke<MasterDataSnapshot>('delete_unit_conversion', { conversionId })
  },

  async convertUnits(input: ConvertUnitsInput): Promise<ConversionResult> {
    if (!isTauriApp()) {
      return desktopFallback.convertUnits(input)
    }

    return invoke<ConversionResult>('convert_units', { input })
  },
}
