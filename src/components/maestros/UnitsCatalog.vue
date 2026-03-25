<script setup lang="ts">
import { ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useMasterDataStore } from '@/stores/masterData'
import type { Unit } from '@/types/masterData'

const emit = defineEmits<{
  (e: 'edit', unit: Unit): void
}>()

const masterDataStore = useMasterDataStore()
const { saving, units } = storeToRefs(masterDataStore)

const deleteDialog = ref(false)
const pendingDeleteUnit = ref<Unit | null>(null)

const requestDeleteUnit = (unit: Unit) => {
  pendingDeleteUnit.value = unit
  deleteDialog.value = true
}

const confirmDeleteUnit = async () => {
  if (!pendingDeleteUnit.value) return
  await masterDataStore.deleteUnit(pendingDeleteUnit.value.id)
  pendingDeleteUnit.value = null
  deleteDialog.value = false
}
</script>

<template>
  <v-card class="mb-6" flat border>
    <v-card-title class="pa-4 font-weight-bold">
      Catalogo de unidades
    </v-card-title>

    <v-table hover>
      <thead>
        <tr>
          <th>Codigo</th>
          <th>Nombre</th>
          <th>Simbolo</th>
          <th class="text-right">Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="unit in units" :key="unit.id" class="cursor-pointer" @click="emit('edit', unit)">
          <td>
            <v-chip size="small" variant="tonal" color="primary">
              {{ unit.code }}
            </v-chip>
          </td>
          <td class="font-weight-medium">{{ unit.name }}</td>
          <td>{{ unit.symbol }}</td>
          <td class="text-right actions-cell">
            <v-btn size="small" variant="text" icon="mdi-pencil-outline" color="primary" @click.stop="emit('edit', unit)" />
            <v-btn size="small" variant="text" icon="mdi-delete-outline" color="error" @click.stop="requestDeleteUnit(unit)" />
          </td>
        </tr>
        <tr v-if="units.length === 0">
          <td colspan="4" class="text-center pa-6 text-medium-emphasis">
            No hay unidades registradas actualmente.
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
        Se eliminara la unidad <strong>{{ pendingDeleteUnit?.name }}</strong>. Si esta en uso, la aplicacion bloqueara la operacion.
      </v-card-text>
      <v-card-actions class="pa-4 bg-grey-lighten-4">
        <v-spacer />
        <v-btn variant="text" @click="deleteDialog = false">Cancelar</v-btn>
        <v-btn color="error" variant="elevated" :loading="saving" @click="confirmDeleteUnit">
          Eliminar
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.actions-cell { white-space: nowrap; }
.cursor-pointer { cursor: pointer; transition: background-color 0.2s; }
.cursor-pointer:hover { background-color: rgb(var(--v-theme-grey-lighten-4)); }
</style>
