<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { storeToRefs } from 'pinia'

import { buildProductionPlan } from '@/lib/productionPlanning'
import { useMasterDataStore } from '@/stores/masterData'
import { useProductionOrdersStore } from '@/stores/productionOrders'
import type { MasterDataSnapshot } from '@/types/masterData'
import type { ProductionOrder, ProductionOrderStatus } from '@/types/production'

const masterDataStore = useMasterDataStore()
const productionOrdersStore = useProductionOrdersStore()

const { conversions, materials, products, recipes, units } = storeToRefs(masterDataStore)
const { error, loading, orders, saving, successMessage } = storeToRefs(productionOrdersStore)

const search = ref('')
const statusFilter = ref<string | null>(null)
const isDialogOpen = ref(false)
const isDetailDialogOpen = ref(false)
const deleteDialog = ref(false)
const orderToDelete = ref<ProductionOrder | null>(null)
const selectedOrder = ref<ProductionOrder | null>(null)
const formError = ref<string | null>(null)
const editingOrder = ref<ProductionOrder | null>(null)

const statusOptions: Array<{ title: string; value: ProductionOrderStatus }> = [
  { title: 'Borrador', value: 'draft' },
  { title: 'Planificada', value: 'planned' },
  { title: 'Liberada', value: 'released' },
  { title: 'Completada', value: 'completed' },
  { title: 'Cancelada', value: 'cancelled' },
]

const form = reactive({
  id: null as number | null,
  orderNumber: '',
  productId: null as number | null,
  plannedQuantity: 1,
  plannedDate: new Date().toISOString().slice(0, 10),
  status: 'planned' as ProductionOrderStatus,
  notes: '',
})

const snapshot = computed<MasterDataSnapshot>(() => ({
  units: units.value,
  materials: materials.value,
  products: products.value,
  recipes: recipes.value,
  conversions: conversions.value,
}))

const productOptions = computed(() =>
  products.value
    .filter((product) => product.status === 'active' && product.recipeId)
    .map((product) => ({
      title: `${product.name} (${product.code})`,
      value: product.id,
    }))
)

const filteredOrders = computed(() => {
  const term = search.value.trim().toLowerCase()
  return orders.value.filter((order) => {
    const product = products.value.find((entry) => entry.id === order.productId)
    const matchesSearch =
      !term ||
      order.orderNumber.toLowerCase().includes(term) ||
      (product?.name ?? '').toLowerCase().includes(term)

    const matchesStatus = !statusFilter.value || order.status === statusFilter.value
    return matchesSearch && matchesStatus
  })
})

const orderPreview = computed(() => {
  if (!form.productId || !Number.isFinite(form.plannedQuantity) || form.plannedQuantity <= 0) return null

  try {
    return buildProductionPlan(snapshot.value, form.productId, form.plannedQuantity)
  } catch {
    return null
  }
})

const summaryCards = computed(() => [
  {
    title: 'Ordenes registradas',
    value: orders.value.length,
    icon: 'mdi-clipboard-text-outline',
  },
  {
    title: 'Ordenes abiertas',
    value: orders.value.filter((order) => ['draft', 'planned', 'released'].includes(order.status)).length,
    icon: 'mdi-progress-clock',
  },
  {
    title: 'Costo estimado abierto',
    value: formatCurrency(
      orders.value
        .filter((order) => ['draft', 'planned', 'released'].includes(order.status))
        .reduce((sum, order) => sum + order.summary.totalEstimatedCost, 0)
    ),
    icon: 'mdi-cash-register',
  },
])

const showSuccess = computed({
  get: () => Boolean(successMessage.value),
  set: (value: boolean) => {
    if (!value) productionOrdersStore.clearSuccess()
  },
})

const resetForm = () => {
  form.id = null
  form.orderNumber = ''
  form.productId = productOptions.value[0]?.value ?? null
  form.plannedQuantity = 1
  form.plannedDate = new Date().toISOString().slice(0, 10)
  form.status = 'planned'
  form.notes = ''
  formError.value = null
}

const openCreateDialog = () => {
  editingOrder.value = null
  resetForm()
  isDialogOpen.value = true
}

const openEditDialog = (order: ProductionOrder) => {
  editingOrder.value = order
  form.id = order.id
  form.orderNumber = order.orderNumber
  form.productId = order.productId
  form.plannedQuantity = order.plannedQuantity
  form.plannedDate = order.plannedDate
  form.status = order.status
  form.notes = order.notes ?? ''
  formError.value = null
  isDialogOpen.value = true
}

const openDetailDialog = (order: ProductionOrder) => {
  selectedOrder.value = order
  isDetailDialogOpen.value = true
}

const requestDelete = (order: ProductionOrder) => {
  orderToDelete.value = order
  deleteDialog.value = true
}

const validateForm = () => {
  if (!form.productId) return 'Selecciona un producto listo para producir.'
  if (!Number.isFinite(form.plannedQuantity) || form.plannedQuantity <= 0) return 'La cantidad planeada debe ser mayor que cero.'
  if (!form.plannedDate) return 'Selecciona la fecha planificada.'
  return null
}

const submitOrder = async () => {
  formError.value = validateForm()
  if (formError.value) return

  const payload = {
    orderNumber: form.orderNumber || null,
    productId: Number(form.productId),
    plannedQuantity: Number(form.plannedQuantity),
    plannedDate: form.plannedDate,
    status: form.status,
    notes: form.notes || null,
  }

  if (editingOrder.value && form.id) {
    await productionOrdersStore.updateOrder({ id: form.id, ...payload }, snapshot.value)
  } else {
    await productionOrdersStore.createOrder(payload, snapshot.value)
  }

  isDialogOpen.value = false
}

const confirmDelete = async () => {
  if (!orderToDelete.value) return
  await productionOrdersStore.deleteOrder(orderToDelete.value.id)
  deleteDialog.value = false
  orderToDelete.value = null
}

const resolveProductName = (productId: number) => products.value.find((product) => product.id === productId)?.name ?? `#${productId}`
const resolveUnitLabel = (unitId: number) => units.value.find((unit) => unit.id === unitId)?.symbol ?? `#${unitId}`
const resolveStatusLabel = (status: string) => statusOptions.find((item) => item.value === status)?.title ?? status
const resolveStatusColor = (status: string) => ({ draft: 'grey', planned: 'info', released: 'warning', completed: 'success', cancelled: 'error' }[status] ?? 'grey')
const resolveRecipeYield = (recipeId: number) => {
  const recipe = recipes.value.find((entry) => entry.id === recipeId)
  if (!recipe) return 'Sin receta'
  return `${recipe.outputQuantity} ${resolveUnitLabel(recipe.outputUnitId)}`
}
const formatCurrency = (value: number | null) =>
  value === null
    ? 'Sin costo'
    : new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(value)

onMounted(async () => {
  const tasks: Promise<unknown>[] = [productionOrdersStore.hydrate()]
  if (!masterDataStore.hasData) {
    tasks.push(masterDataStore.hydrate())
  }
  await Promise.all(tasks)

  if (!form.productId) {
    form.productId = productOptions.value[0]?.value ?? null
  }
})
</script>

<template>
  <section class="production-tab">
    <div class="d-flex justify-space-between align-center ga-3 flex-wrap mb-6">
      <div>
        <h2 class="text-h5 mb-1">Ordenes de produccion</h2>
        <p class="text-body-2 text-medium-emphasis">
          Genera cada orden a partir de la receta tecnica y obten el requerimiento de materiales y costo estimado.
        </p>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openCreateDialog">Nueva orden</v-btn>
    </div>

    <v-row class="mb-4" dense>
      <v-col v-for="card in summaryCards" :key="card.title" cols="12" md="4">
        <v-card border flat rounded="lg" class="pa-4">
          <div class="d-flex align-center justify-space-between">
            <div>
              <div class="text-overline text-medium-emphasis">{{ card.title }}</div>
              <div class="text-h5 font-weight-bold">{{ card.value }}</div>
            </div>
            <v-avatar color="primary" variant="tonal">
              <v-icon :icon="card.icon" />
            </v-avatar>
          </div>
        </v-card>
      </v-col>
    </v-row>

    <v-expand-transition>
      <v-alert
        v-if="error"
        type="error"
        variant="tonal"
        closable
        class="mb-4"
        border="start"
        @click:close="productionOrdersStore.clearError()"
      >
        {{ error }}
      </v-alert>
    </v-expand-transition>

    <v-card border flat rounded="lg">
      <v-card-title class="catalog-header pa-4">
        <span class="font-weight-bold">Planeacion</span>
        <div class="catalog-filters">
          <v-text-field v-model="search" label="Buscar" prepend-inner-icon="mdi-magnify" density="compact" variant="outlined" hide-details class="search-field" />
          <v-select v-model="statusFilter" :items="statusOptions" label="Estado" density="compact" variant="outlined" hide-details clearable class="filter-field" />
        </div>
      </v-card-title>

      <v-progress-linear v-if="loading" indeterminate />

      <v-table v-else hover>
        <thead>
          <tr>
            <th>Orden</th>
            <th>Producto</th>
            <th>Fecha</th>
            <th>Estado</th>
            <th>Resumen</th>
            <th class="text-right">Acciones</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="order in filteredOrders" :key="order.id">
            <td>
              <div class="font-weight-medium">{{ order.orderNumber }}</div>
              <div class="text-caption text-medium-emphasis">{{ order.plannedQuantity }} {{ resolveUnitLabel(order.plannedUnitId) }}</div>
            </td>
            <td>{{ resolveProductName(order.productId) }}</td>
            <td>{{ order.plannedDate }}</td>
            <td>
              <v-chip size="small" :color="resolveStatusColor(order.status)" variant="flat">
                {{ resolveStatusLabel(order.status) }}
              </v-chip>
            </td>
            <td>
              <div class="text-body-2">{{ order.summary.totalMaterials }} materiales</div>
              <div class="text-caption text-medium-emphasis">{{ formatCurrency(order.summary.totalEstimatedCost) }}</div>
            </td>
            <td class="text-right actions-cell">
              <v-btn size="small" variant="text" color="primary" prepend-icon="mdi-eye-outline" @click="openDetailDialog(order)">
                Ver detalle
              </v-btn>
              <v-btn size="small" variant="text" icon="mdi-pencil-outline" color="primary" @click="openEditDialog(order)" />
              <v-btn size="small" variant="text" icon="mdi-delete-outline" color="error" @click="requestDelete(order)" />
            </td>
          </tr>
          <tr v-if="filteredOrders.length === 0">
            <td colspan="6" class="empty-cell">Crea la primera orden de produccion desde esta vista.</td>
          </tr>
        </tbody>
      </v-table>
    </v-card>

    <v-dialog v-model="isDialogOpen" max-width="900" scrollable transition="dialog-bottom-transition">
      <v-card rounded="xl">
        <v-card-title class="pa-4 d-flex align-center">
          <v-icon icon="mdi-clipboard-edit-outline" class="mr-3" color="primary" />
          <span class="text-h6 font-weight-bold">{{ editingOrder ? 'Editar orden' : 'Nueva orden de produccion' }}</span>
          <v-spacer />
          <v-btn icon="mdi-close" variant="text" @click="isDialogOpen = false" />
        </v-card-title>
        <v-divider />
        <v-card-text class="pa-6">
          <div class="d-grid ga-4">
            <v-alert v-if="formError" type="warning" variant="tonal">{{ formError }}</v-alert>

            <v-row dense>
              <v-col cols="12" md="4">
                <v-text-field v-model="form.orderNumber" label="Numero de orden" hint="Si lo dejas vacio se genera automaticamente." persistent-hint variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="5">
                <v-select v-model="form.productId" :items="productOptions" label="Producto a fabricar" variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="3">
                <v-select v-model="form.status" :items="statusOptions" label="Estado" variant="outlined" density="comfortable" />
              </v-col>
            </v-row>

            <v-row dense>
              <v-col cols="12" md="4">
                <v-text-field v-model.number="form.plannedQuantity" label="Cantidad a producir" type="number" min="0.0001" step="0.0001" variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="4">
                <v-text-field v-model="form.plannedDate" label="Fecha planificada" type="date" variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="4">
                <v-text-field
                  :model-value="orderPreview ? resolveUnitLabel(orderPreview.plannedUnitId) : 'Selecciona un producto'"
                  label="Unidad de produccion"
                  variant="outlined"
                  density="comfortable"
                  readonly
                />
              </v-col>
            </v-row>

            <v-textarea v-model="form.notes" label="Notas de la orden" rows="2" auto-grow variant="outlined" density="comfortable" />

            <v-divider />

            <v-alert v-if="orderPreview" type="info" variant="tonal">
              Esta orden usara la receta asociada al producto y generara {{ orderPreview.materials.length }} material(es) con costo estimado de
              <strong>{{ formatCurrency(orderPreview.summary.totalEstimatedCost) }}</strong>.
            </v-alert>

            <v-table v-if="orderPreview" hover>
              <thead>
                <tr>
                  <th>Material</th>
                  <th>Consumo</th>
                  <th>Stock requerido</th>
                  <th>Costo estimado</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="line in orderPreview.materials" :key="line.materialId">
                  <td>{{ line.materialName }}</td>
                  <td>{{ line.requiredQuantity }} {{ resolveUnitLabel(line.requiredUnitId) }}</td>
                  <td>{{ line.stockRequiredQuantity }} {{ resolveUnitLabel(line.stockUnitId) }}</td>
                  <td>{{ formatCurrency(line.estimatedLineCost) }}</td>
                </tr>
              </tbody>
            </v-table>
          </div>
        </v-card-text>
        <v-divider />
        <v-card-actions class="pa-4 bg-grey-lighten-4">
          <v-btn variant="text" @click="isDialogOpen = false">Cancelar</v-btn>
          <v-spacer />
          <v-btn color="primary" :loading="saving" prepend-icon="mdi-check" @click="submitOrder">Guardar orden</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="isDetailDialogOpen" max-width="1100" scrollable transition="dialog-bottom-transition">
      <v-card v-if="selectedOrder" rounded="xl">
        <v-card-title class="pa-4 d-flex align-center flex-wrap ga-3">
          <div>
            <div class="text-h6 font-weight-bold">{{ selectedOrder.orderNumber }}</div>
            <div class="text-body-2 text-medium-emphasis">
              {{ resolveProductName(selectedOrder.productId) }} · {{ selectedOrder.plannedQuantity }} {{ resolveUnitLabel(selectedOrder.plannedUnitId) }} · {{ selectedOrder.plannedDate }}
            </div>
          </div>
          <v-spacer />
          <v-chip :color="resolveStatusColor(selectedOrder.status)" variant="flat">
            {{ resolveStatusLabel(selectedOrder.status) }}
          </v-chip>
          <v-btn icon="mdi-close" variant="text" @click="isDetailDialogOpen = false" />
        </v-card-title>

        <v-divider />

        <v-card-text class="pa-6">
          <v-row class="mb-4" dense>
            <v-col cols="12" md="4">
              <v-card variant="outlined" rounded="lg">
                <v-card-text>
                  <div class="text-overline text-medium-emphasis">Receta</div>
                  <div class="font-weight-bold">{{ resolveRecipeYield(selectedOrder.recipeId) }}</div>
                </v-card-text>
              </v-card>
            </v-col>
            <v-col cols="12" md="4">
              <v-card variant="outlined" rounded="lg">
                <v-card-text>
                  <div class="text-overline text-medium-emphasis">Materiales</div>
                  <div class="font-weight-bold">{{ selectedOrder.summary.totalMaterials }}</div>
                </v-card-text>
              </v-card>
            </v-col>
            <v-col cols="12" md="4">
              <v-card variant="outlined" rounded="lg">
                <v-card-text>
                  <div class="text-overline text-medium-emphasis">Costo estimado</div>
                  <div class="font-weight-bold">{{ formatCurrency(selectedOrder.summary.totalEstimatedCost) }}</div>
                </v-card-text>
              </v-card>
            </v-col>
          </v-row>

          <v-alert v-if="selectedOrder.summary.missingCostLines > 0" type="warning" variant="tonal" class="mb-4">
            Hay {{ selectedOrder.summary.missingCostLines }} material(es) sin costo estimado. Registra su costo en Maestros para completar la valorizacion.
          </v-alert>

          <v-table hover>
            <thead>
              <tr>
                <th>Material</th>
                <th>Consumo</th>
                <th>Stock requerido</th>
                <th>Compra sugerida</th>
                <th>Costo unitario</th>
                <th>Costo linea</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="line in selectedOrder.materials" :key="line.materialId">
                <td>
                  <div class="font-weight-medium">{{ line.materialName }}</div>
                  <div class="text-caption text-medium-emphasis">{{ line.materialCode }}</div>
                </td>
                <td>{{ line.requiredQuantity }} {{ resolveUnitLabel(line.requiredUnitId) }}</td>
                <td>{{ line.stockRequiredQuantity }} {{ resolveUnitLabel(line.stockUnitId) }}</td>
                <td>
                  {{
                    line.purchaseSuggestedQuantity === null || line.purchaseUnitId === null
                      ? 'Sin conversion'
                      : `${line.purchaseSuggestedQuantity} ${resolveUnitLabel(line.purchaseUnitId)}`
                  }}
                </td>
                <td>
                  {{
                    line.estimatedUnitCost === null || line.estimatedCostUnitId === null
                      ? 'Sin costo'
                      : `${formatCurrency(line.estimatedUnitCost)} / ${resolveUnitLabel(line.estimatedCostUnitId)}`
                  }}
                </td>
                <td>{{ formatCurrency(line.estimatedLineCost) }}</td>
              </tr>
            </tbody>
          </v-table>

          <div v-if="selectedOrder.notes" class="text-body-2 text-medium-emphasis mt-4">
            Nota: {{ selectedOrder.notes }}
          </div>
        </v-card-text>
      </v-card>
    </v-dialog>

    <v-dialog v-model="deleteDialog" max-width="420">
      <v-card rounded="xl">
        <v-card-title class="text-error d-flex align-center pa-4">
          <v-icon icon="mdi-alert" class="mr-2" /> Confirmar eliminacion
        </v-card-title>
        <v-card-text class="pa-4 pt-0">
          Se eliminara la orden <strong>{{ orderToDelete?.orderNumber }}</strong>.
        </v-card-text>
        <v-card-actions class="pa-4 bg-grey-lighten-4">
          <v-spacer />
          <v-btn variant="text" @click="deleteDialog = false">Cancelar</v-btn>
          <v-btn color="error" variant="elevated" :loading="saving" @click="confirmDelete">Eliminar</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-snackbar v-model="showSuccess" color="success" location="top right" variant="flat" elevation="12" rounded="lg">
      <div class="d-flex align-center ga-2">
        <v-icon icon="mdi-check-circle" />
        <span class="font-weight-medium">{{ successMessage }}</span>
      </div>
    </v-snackbar>
  </section>
</template>

<style scoped>
.production-tab {
  display: grid;
  gap: 16px;
}

.catalog-header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: center;
}

.catalog-filters {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.filter-field {
  min-width: 150px;
}

.search-field {
  min-width: 220px;
}

.actions-cell {
  white-space: nowrap;
}

.empty-cell {
  padding: 32px 16px;
  text-align: center;
  color: rgb(var(--v-theme-on-surface));
  opacity: 0.7;
}
</style>
