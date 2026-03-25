<script setup lang="ts">
import { ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { UnitConversion } from '@/types/masterData'

const emit = defineEmits<{
  (e: 'edit', conversion: UnitConversion): void
}>()

const masterDataStore = useMasterDataStore()
const { conversions, saving, units } = storeToRefs(masterDataStore)

const deleteDialog = ref(false)
const pendingDeleteConversion = ref<UnitConversion | null>(null)

const requestDelete = (conversion: UnitConversion) => {
  pendingDeleteConversion.value = conversion
  deleteDialog.value = true
}

const confirmDelete = async () => {
  if (!pendingDeleteConversion.value) return
  await masterDataStore.deleteUnitConversion(pendingDeleteConversion.value.id)
  pendingDeleteConversion.value = null
  deleteDialog.value = false
}

const resolveUnitLabel = (unitId: number) => units.value.find((unit) => unit.id === unitId)?.symbol ?? `#${unitId}`
</script>

<template>
  <v-card flat border>
    <v-card-title class="font-weight-bold pa-4">
      Conversiones registradas
    </v-card-title>
    <v-card-text class="pt-0 text-body-2 text-medium-emphasis">
      Usa el boton flotante para crear conversiones nuevas o haz clic sobre una fila para editarla.
    </v-card-text>
    <v-table hover>
      <thead>
        <tr>
          <th>Desde</th>
          <th>Hacia</th>
          <th>Factor</th>
          <th class="text-right">Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="conversion in conversions" :key="conversion.id" class="cursor-pointer" @click="emit('edit', conversion)">
          <td>
            <v-chip size="small" variant="tonal" color="primary">
              1 {{ resolveUnitLabel(conversion.fromUnitId) }}
            </v-chip>
          </td>
          <td>
            <v-chip size="small" variant="tonal">
              {{ resolveUnitLabel(conversion.toUnitId) }}
            </v-chip>
          </td>
          <td class="font-weight-bold">{{ conversion.factor }}</td>
          <td class="text-right actions-cell">
            <v-btn size="small" variant="text" icon="mdi-pencil-outline" color="primary" @click.stop="emit('edit', conversion)" />
            <v-btn size="small" variant="text" icon="mdi-delete-outline" color="error" @click.stop="requestDelete(conversion)" />
          </td>
        </tr>
        <tr v-if="conversions.length === 0">
          <td colspan="4" class="text-center pa-6 text-medium-emphasis">
            No hay conversiones registradas actualmente.
          </td>
        </tr>
      </tbody>
    </v-table>
  </v-card>

  <v-dialog v-model="deleteDialog" max-width="400">
    <v-card rounded="xl">
      <v-card-title class="text-error d-flex align-center pa-4">
        <v-icon icon="mdi-alert" class="mr-2" />
        Confirmar eliminacion
      </v-card-title>
      <v-card-text class="pa-4 pt-0">
        Se eliminara la conversion entre <strong>{{ pendingDeleteConversion ? resolveUnitLabel(pendingDeleteConversion.fromUnitId) : '' }}</strong> y <strong>{{ pendingDeleteConversion ? resolveUnitLabel(pendingDeleteConversion.toUnitId) : '' }}</strong>.
      </v-card-text>
      <v-card-actions class="pa-4 bg-grey-lighten-4">
        <v-spacer />
        <v-btn variant="text" @click="deleteDialog = false">Cancelar</v-btn>
        <v-btn color="error" variant="elevated" :loading="saving" @click="confirmDelete">Eliminar</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.actions-cell { white-space: nowrap; }
.cursor-pointer { cursor: pointer; transition: background-color 0.2s; }
.cursor-pointer:hover { background-color: rgb(var(--v-theme-grey-lighten-4)); }
</style>
