<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'

import { useMasterDataStore } from '@/stores/masterData'
import { useProductionOrdersStore } from '@/stores/productionOrders'

const masterDataStore = useMasterDataStore()
const productionOrdersStore = useProductionOrdersStore()

const { units } = storeToRefs(masterDataStore)
const { activeOrders } = storeToRefs(productionOrdersStore)

const consolidatedMaterials = computed(() => {
  const map = new Map<number, {
    materialId: number
    materialName: string
    materialCode: string
    stockRequiredQuantity: number
    stockUnitId: number
    purchaseSuggestedQuantity: number | null
    purchaseUnitId: number | null
    totalEstimatedCost: number
    missingCostLines: number
    orderCount: number
  }>()

  for (const order of activeOrders.value) {
    for (const line of order.materials) {
      const current = map.get(line.materialId)
      if (current) {
        current.stockRequiredQuantity += line.stockRequiredQuantity
        current.purchaseSuggestedQuantity =
          current.purchaseSuggestedQuantity !== null && line.purchaseSuggestedQuantity !== null
            ? current.purchaseSuggestedQuantity + line.purchaseSuggestedQuantity
            : null
        current.totalEstimatedCost += line.estimatedLineCost ?? 0
        current.missingCostLines += line.estimatedLineCost === null ? 1 : 0
        current.orderCount += 1
      } else {
        map.set(line.materialId, {
          materialId: line.materialId,
          materialName: line.materialName,
          materialCode: line.materialCode,
          stockRequiredQuantity: line.stockRequiredQuantity,
          stockUnitId: line.stockUnitId,
          purchaseSuggestedQuantity: line.purchaseSuggestedQuantity,
          purchaseUnitId: line.purchaseUnitId,
          totalEstimatedCost: line.estimatedLineCost ?? 0,
          missingCostLines: line.estimatedLineCost === null ? 1 : 0,
          orderCount: 1,
        })
      }
    }
  }

  return [...map.values()].sort((a, b) => a.materialName.localeCompare(b.materialName))
})

const totalEstimatedCost = computed(() => consolidatedMaterials.value.reduce((sum, line) => sum + line.totalEstimatedCost, 0))
const unresolvedCosts = computed(() => consolidatedMaterials.value.filter((line) => line.missingCostLines > 0).length)

const resolveUnitLabel = (unitId: number | null) => units.value.find((unit) => unit.id === unitId)?.symbol ?? (unitId ? `#${unitId}` : '-')
const formatCurrency = (value: number) =>
  new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(value)

onMounted(async () => {
  const tasks: Promise<unknown>[] = [productionOrdersStore.hydrate()]
  if (!masterDataStore.hasData) {
    tasks.push(masterDataStore.hydrate())
  }
  await Promise.all(tasks)
})
</script>

<template>
  <section class="production-tab">
    <div class="mb-4">
      <h2 class="text-h5 mb-1">Materiales requeridos</h2>
      <p class="text-body-2 text-medium-emphasis">
        Consolida la demanda de materias primas de las ordenes abiertas para compras y abastecimiento.
      </p>
    </div>

    <v-row class="mb-4" dense>
      <v-col cols="12" md="4">
        <v-card border flat rounded="lg">
          <v-card-text>
            <div class="text-overline text-medium-emphasis">Ordenes abiertas</div>
            <div class="text-h5 font-weight-bold">{{ activeOrders.length }}</div>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="12" md="4">
        <v-card border flat rounded="lg">
          <v-card-text>
            <div class="text-overline text-medium-emphasis">Materiales consolidados</div>
            <div class="text-h5 font-weight-bold">{{ consolidatedMaterials.length }}</div>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="12" md="4">
        <v-card border flat rounded="lg">
          <v-card-text>
            <div class="text-overline text-medium-emphasis">Costo estimado total</div>
            <div class="text-h5 font-weight-bold">{{ formatCurrency(totalEstimatedCost) }}</div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-alert v-if="unresolvedCosts > 0" type="warning" variant="tonal" class="mb-4">
      Hay {{ unresolvedCosts }} material(es) sin costo suficiente para valorizacion completa.
    </v-alert>

    <v-card border flat rounded="lg">
      <v-table hover>
        <thead>
          <tr>
            <th>Material</th>
            <th>Stock requerido</th>
            <th>Compra sugerida</th>
            <th>Ordenes</th>
            <th>Costo acumulado</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="line in consolidatedMaterials" :key="line.materialId">
            <td>
              <div class="font-weight-medium">{{ line.materialName }}</div>
              <div class="text-caption text-medium-emphasis">{{ line.materialCode }}</div>
            </td>
            <td>{{ line.stockRequiredQuantity.toFixed(4) }} {{ resolveUnitLabel(line.stockUnitId) }}</td>
            <td>
              {{
                line.purchaseSuggestedQuantity === null || line.purchaseUnitId === null
                  ? 'Sin conversion'
                  : `${line.purchaseSuggestedQuantity.toFixed(4)} ${resolveUnitLabel(line.purchaseUnitId)}`
              }}
            </td>
            <td>{{ line.orderCount }}</td>
            <td>{{ line.missingCostLines > 0 ? 'Pendiente parcial' : formatCurrency(line.totalEstimatedCost) }}</td>
          </tr>
          <tr v-if="consolidatedMaterials.length === 0">
            <td colspan="5" class="empty-cell">No hay ordenes abiertas para consolidar materiales.</td>
          </tr>
        </tbody>
      </v-table>
    </v-card>
  </section>
</template>

<style scoped>
.production-tab {
  display: grid;
  gap: 16px;
}

.empty-cell {
  padding: 32px 16px;
  text-align: center;
  color: rgb(var(--v-theme-on-surface));
  opacity: 0.6;
}
</style>
