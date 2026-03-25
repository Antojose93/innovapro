<template>
  <section class="production-tab">
    <!-- Encabezado -->
    <div class="mb-6">
      <h2 class="text-h5 mb-1">Unidades de medida</h2>
      <p class="text-body-2 text-medium-emphasis">
        Define y consulta las unidades base que usa el proceso productivo.
      </p>
    </div>

    <v-row>
      <!-- Catálogo base -->
      <v-col cols="12" md="6">
        <v-card variant="outlined" rounded="lg">
          <v-card-title class="text-overline text-medium-emphasis px-4 pt-4 pb-2">
            Catálogo base
          </v-card-title>

          <v-list lines="two" class="px-2 pb-2">
            <v-list-item
              v-for="unit in units"
              :key="unit.title"
              :title="unit.title"
              :subtitle="unit.subtitle"
              rounded="lg"
            >
              <template #append>
                <v-chip
                  color="success"
                  size="x-small"
                  variant="tonal"
                  label
                >
                  Activo
                </v-chip>
              </template>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>

      <!-- Acciones -->
      <v-col cols="12" md="6">
        <v-card variant="outlined" rounded="lg">
          <v-card-title class="text-overline text-medium-emphasis px-4 pt-4 pb-2">
            Acciones
          </v-card-title>

          <v-list class="px-2 pb-2">
            <v-list-item
              v-for="action in actions"
              :key="action.title"
              :prepend-icon="action.icon"
              :title="action.title"
              :subtitle="action.subtitle"
              :active="false"
              rounded="lg"
              class="action-item mb-1"
              link
              @click="handleAction(action)"
            >
              <template #append>
                <v-icon size="small" color="medium-emphasis">mdi-chevron-right</v-icon>
              </template>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
    </v-row>

    <!-- Snackbar de feedback -->
    <v-snackbar
      v-model="snackbar.show"
      :timeout="2500"
      location="bottom center"
      variant="tonal"
      color="surface-variant"
      rounded="lg"
    >
      <v-icon :icon="snackbar.icon" size="small" class="mr-2" />
      {{ snackbar.message }}
    </v-snackbar>
  </section>
</template>

<script setup>
import { ref, reactive } from 'vue'

// --- Datos del catálogo ---
const units = [
  { title: 'Kilogramo', subtitle: 'Materia prima y mezclas' },
  { title: 'Litro',     subtitle: 'Líquidos e insumos' },
  { title: 'Unidad',   subtitle: 'Producto terminado' },
]

// --- Acciones con subtítulos descriptivos ---
const actions = [
  {
    title:    'Registrar nueva unidad',
    subtitle: 'Agrega una unidad al catálogo',
    icon:     'mdi-plus-circle-outline',
    feedback: 'Abriendo formulario de registro...',
    feedbackIcon: 'mdi-plus-circle-outline',
  },
  {
    title:    'Configurar conversiones',
    subtitle: 'Define equivalencias entre unidades',
    icon:     'mdi-swap-horizontal',
    feedback: 'Abriendo tabla de conversiones...',
    feedbackIcon: 'mdi-swap-horizontal',
  },
  {
    title:    'Validar unidad por producto',
    subtitle: 'Verifica la unidad asignada a cada producto',
    icon:     'mdi-check-decagram-outline',
    feedback: 'Abriendo validación por producto...',
    feedbackIcon: 'mdi-check-decagram-outline',
  },
]

// --- Feedback visual (snackbar) ---
const snackbar = reactive({
  show: false,
  message: '',
  icon: '',
})

function handleAction(action) {
  snackbar.message = action.feedback
  snackbar.icon = action.feedbackIcon
  snackbar.show = true

  // TODO: reemplaza esto con la navegación o lógica real
  // router.push({ name: action.route })
}
</script>

<style scoped>
.action-item {
  cursor: pointer;
  transition: background-color 0.15s ease;
}

/* Separación visual entre secciones */
.production-tab {
  max-width: 900px;
}
</style>