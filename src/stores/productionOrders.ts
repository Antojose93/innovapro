import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import { buildProductionPlan } from '@/lib/productionPlanning'
import type { MasterDataSnapshot } from '@/types/masterData'
import type { CreateProductionOrderInput, ProductionOrder, UpdateProductionOrderInput } from '@/types/production'

const STORAGE_KEY = 'erp-production-orders'

const normalizeText = (value?: string | null) => {
  const trimmed = value?.trim()
  return trimmed ? trimmed : null
}

export const useProductionOrdersStore = defineStore('production-orders', () => {
  const orders = ref<ProductionOrder[]>([])
  const loading = ref(false)
  const saving = ref(false)
  const error = ref<string | null>(null)
  const successMessage = ref<string | null>(null)

  const activeOrders = computed(() =>
    orders.value.filter((order) => ['draft', 'planned', 'released'].includes(order.status))
  )

  const persist = () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(orders.value))
  }

  const hydrate = async () => {
    loading.value = true
    error.value = null

    try {
      const raw = localStorage.getItem(STORAGE_KEY)
      orders.value = raw ? (JSON.parse(raw) as ProductionOrder[]) : []
    } catch (hydrateError) {
      console.error('No fue posible cargar las ordenes de produccion', hydrateError)
      orders.value = []
      error.value = 'No fue posible leer las ordenes guardadas.'
    } finally {
      loading.value = false
    }
  }

  const clearError = () => {
    error.value = null
  }

  const clearSuccess = () => {
    successMessage.value = null
  }

  const buildOrderNumber = (plannedDate: string) => {
    const compactDate = plannedDate.replace(/-/g, '')
    const nextSequence = orders.value.length + 1
    return `OP-${compactDate}-${String(nextSequence).padStart(3, '0')}`
  }

  const createOrder = async (input: CreateProductionOrderInput, snapshot: MasterDataSnapshot) => {
    saving.value = true
    error.value = null

    try {
      const nextId = orders.value.reduce((maxId, order) => Math.max(maxId, order.id), 0) + 1
      const plan = buildProductionPlan(snapshot, input.productId, input.plannedQuantity)
      const now = new Date().toISOString()

      const order: ProductionOrder = {
        id: nextId,
        orderNumber: normalizeText(input.orderNumber) ?? buildOrderNumber(input.plannedDate),
        productId: input.productId,
        recipeId: plan.recipeId,
        plannedQuantity: input.plannedQuantity,
        plannedUnitId: plan.plannedUnitId,
        plannedDate: input.plannedDate,
        status: input.status,
        notes: normalizeText(input.notes),
        materials: plan.materials,
        summary: plan.summary,
        createdAt: now,
        updatedAt: now,
      }

      orders.value = [order, ...orders.value]
      persist()
      successMessage.value = 'Orden de produccion creada correctamente.'
    } catch (saveError) {
      console.error('No fue posible crear la orden', saveError)
      error.value = saveError instanceof Error ? saveError.message : 'No fue posible crear la orden.'
      throw saveError
    } finally {
      saving.value = false
    }
  }

  const updateOrder = async (input: UpdateProductionOrderInput, snapshot: MasterDataSnapshot) => {
    saving.value = true
    error.value = null

    try {
      const current = orders.value.find((order) => order.id === input.id)
      if (!current) {
        throw new Error('La orden seleccionada no existe.')
      }

      const plan = buildProductionPlan(snapshot, input.productId, input.plannedQuantity)
      const updatedAt = new Date().toISOString()

      orders.value = orders.value.map((order) =>
        order.id === input.id
          ? {
              ...order,
              orderNumber: normalizeText(input.orderNumber) ?? order.orderNumber,
              productId: input.productId,
              recipeId: plan.recipeId,
              plannedQuantity: input.plannedQuantity,
              plannedUnitId: plan.plannedUnitId,
              plannedDate: input.plannedDate,
              status: input.status,
              notes: normalizeText(input.notes),
              materials: plan.materials,
              summary: plan.summary,
              updatedAt,
            }
          : order
      )

      persist()
      successMessage.value = 'Orden de produccion actualizada correctamente.'
    } catch (saveError) {
      console.error('No fue posible actualizar la orden', saveError)
      error.value = saveError instanceof Error ? saveError.message : 'No fue posible actualizar la orden.'
      throw saveError
    } finally {
      saving.value = false
    }
  }

  const deleteOrder = async (orderId: number) => {
    saving.value = true
    error.value = null

    try {
      orders.value = orders.value.filter((order) => order.id !== orderId)
      persist()
      successMessage.value = 'Orden de produccion eliminada correctamente.'
    } catch (deleteError) {
      console.error('No fue posible eliminar la orden', deleteError)
      error.value = 'No fue posible eliminar la orden.'
      throw deleteError
    } finally {
      saving.value = false
    }
  }

  return {
    orders,
    activeOrders,
    loading,
    saving,
    error,
    successMessage,
    hydrate,
    clearError,
    clearSuccess,
    createOrder,
    updateOrder,
    deleteOrder,
  }
})
