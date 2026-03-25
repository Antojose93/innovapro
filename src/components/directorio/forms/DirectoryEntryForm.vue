<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'

import { useDirectoryStore } from '@/stores/directory'
import type { DirectoryEntry, DirectoryEntityType } from '@/types/directory'

const props = defineProps<{
  type: DirectoryEntityType
  entryToEdit?: DirectoryEntry | null
}>()

const emit = defineEmits(['success'])

const directoryStore = useDirectoryStore()

const formRef = ref<any>(null)
const formError = ref<string | null>(null)

const isEditMode = computed(() => Boolean(props.entryToEdit))

const titleMap = {
  client: 'cliente',
  supplier: 'proveedor',
}

const statusOptions = [
  { title: 'Activo', value: 'active' },
  { title: 'Prospecto', value: 'prospect' },
  { title: 'Inactivo', value: 'inactive' },
  { title: 'Bloqueado', value: 'blocked' },
]

const suggestedSegments = computed(() =>
  props.type === 'client'
    ? ['Mayorista', 'Minorista', 'Distribuidor', 'Corporativo', 'Institucional']
    : ['Harinas', 'Empaques', 'Logistica', 'Servicios', 'Mantenimiento']
)

const form = reactive({
  id: null as number | null,
  status: 'active',
  displayName: '',
  segment: '',
  legalName: '',
  contactName: '',
  documentType: '',
  documentNumber: '',
  taxId: '',
  phone: '',
  mobile: '',
  email: '',
  secondaryEmail: '',
  website: '',
  country: 'Colombia',
  city: '',
  address: '',
  paymentTerms: '',
  paymentMethod: '',
  creditLimit: null as number | null,
  taxCondition: '',
  bankReference: '',
  notes: '',
  tagsText: '',
})

watch(
  () => props.entryToEdit,
  (entry) => {
    if (entry) {
      form.id = entry.id
      form.status = entry.status
      form.displayName = entry.displayName
      form.segment = entry.segment
      form.legalName = entry.legalName ?? ''
      form.contactName = entry.contactName ?? ''
      form.documentType = entry.documentType ?? ''
      form.documentNumber = entry.documentNumber ?? ''
      form.taxId = entry.taxId ?? ''
      form.phone = entry.phone ?? ''
      form.mobile = entry.mobile ?? ''
      form.email = entry.email ?? ''
      form.secondaryEmail = entry.secondaryEmail ?? ''
      form.website = entry.website ?? ''
      form.country = entry.country ?? 'Colombia'
      form.city = entry.city ?? ''
      form.address = entry.address ?? ''
      form.paymentTerms = entry.paymentTerms ?? ''
      form.paymentMethod = entry.paymentMethod ?? ''
      form.creditLimit = entry.creditLimit
      form.taxCondition = entry.taxCondition ?? ''
      form.bankReference = entry.bankReference ?? ''
      form.notes = entry.notes ?? ''
      form.tagsText = entry.tags.join(', ')
    } else {
      form.id = null
      form.status = 'active'
      form.displayName = ''
      form.segment = ''
      form.legalName = ''
      form.contactName = ''
      form.documentType = ''
      form.documentNumber = ''
      form.taxId = ''
      form.phone = ''
      form.mobile = ''
      form.email = ''
      form.secondaryEmail = ''
      form.website = ''
      form.country = 'Colombia'
      form.city = ''
      form.address = ''
      form.paymentTerms = ''
      form.paymentMethod = ''
      form.creditLimit = null
      form.taxCondition = ''
      form.bankReference = ''
      form.notes = ''
      form.tagsText = ''
    }

    formError.value = null
  },
  { immediate: true }
)

const requiredFields = computed(() =>
  props.type === 'client'
    ? ['Nombre comercial o razon visible', 'Segmento comercial', 'Estado']
    : ['Nombre comercial o razon visible', 'Categoria o segmento de suministro', 'Estado']
)

const optionalFields = computed(() => [
  'Razon social',
  'Contacto principal',
  'Documento',
  'Telefonos',
  'Correos',
  'Ubicacion',
  'Condiciones de pago',
  props.type === 'client' ? 'Cupo de credito' : 'Referencia bancaria',
  'Etiquetas y notas',
])

const validateForm = () => {
  if (!form.displayName.trim()) return `Debes ingresar el nombre base del ${titleMap[props.type]}.`
  if (!form.segment.trim()) return `Debes ingresar el segmento del ${titleMap[props.type]}.`
  if (!form.status) return `Selecciona el estado del ${titleMap[props.type]}.`
  return null
}

const submit = async () => {
  formError.value = validateForm()
  if (formError.value) return false

  const payload = {
    type: props.type,
    status: form.status as 'active' | 'prospect' | 'inactive' | 'blocked',
    displayName: form.displayName,
    segment: form.segment,
    legalName: form.legalName || null,
    contactName: form.contactName || null,
    documentType: form.documentType || null,
    documentNumber: form.documentNumber || null,
    taxId: form.taxId || null,
    phone: form.phone || null,
    mobile: form.mobile || null,
    email: form.email || null,
    secondaryEmail: form.secondaryEmail || null,
    website: form.website || null,
    country: form.country || null,
    city: form.city || null,
    address: form.address || null,
    paymentTerms: form.paymentTerms || null,
    paymentMethod: form.paymentMethod || null,
    creditLimit: form.creditLimit,
    taxCondition: form.taxCondition || null,
    bankReference: form.bankReference || null,
    notes: form.notes || null,
    tags: form.tagsText.split(','),
  }

  try {
    if (isEditMode.value && form.id) {
      await directoryStore.updateEntry({ id: form.id, ...payload })
    } else {
      await directoryStore.createEntry(payload)
    }
    emit('success')
    return true
  } catch (error) {
    console.error('Error al guardar registro del directorio', error)
    return false
  }
}

defineExpose({ submit })
</script>

<template>
  <v-form ref="formRef" @submit.prevent>
    <div class="d-grid ga-4">
      <v-alert v-if="formError" type="warning" variant="tonal">
        {{ formError }}
      </v-alert>

      <v-alert type="info" variant="tonal">
        <div class="font-weight-medium mb-2">Campos obligatorios</div>
        <div>{{ requiredFields.join(' · ') }}</div>
        <div class="mt-2 text-medium-emphasis">Resto de campos opcionales: {{ optionalFields.join(' · ') }}</div>
      </v-alert>

      <v-row dense>
        <v-col cols="12" md="8">
          <v-text-field v-model="form.displayName" label="Nombre visible *" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="4">
          <v-select v-model="form.status" :items="statusOptions" label="Estado *" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="6">
          <v-combobox
            v-model="form.segment"
            :items="suggestedSegments"
            :label="props.type === 'client' ? 'Segmento comercial *' : 'Categoria de suministro *'"
            variant="outlined"
            density="comfortable"
          />
        </v-col>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.legalName" label="Razon social" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.contactName" label="Contacto principal" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="6" md="3">
          <v-text-field v-model="form.documentType" label="Tipo doc." variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="6" md="3">
          <v-text-field v-model="form.documentNumber" label="Numero doc." variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.phone" label="Telefono" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.mobile" label="Celular" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.taxId" label="Condicion fiscal" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.email" label="Correo principal" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="6">
          <v-text-field v-model="form.secondaryEmail" label="Correo alterno" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.country" label="Pais" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.city" label="Ciudad" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.website" label="Sitio web" variant="outlined" density="comfortable" />
        </v-col>
      </v-row>

      <v-textarea v-model="form.address" label="Direccion" rows="2" auto-grow variant="outlined" density="comfortable" />

      <v-divider class="my-2" />

      <v-row dense>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.paymentTerms" label="Plazo de pago" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="4">
          <v-text-field v-model="form.paymentMethod" label="Medio de pago habitual" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" md="4">
          <v-text-field
            v-model="form.bankReference"
            :label="props.type === 'client' ? 'Referencia bancaria' : 'Cuenta o referencia bancaria'"
            variant="outlined"
            density="comfortable"
          />
        </v-col>
      </v-row>

      <v-row dense>
        <v-col cols="12" md="6" v-if="props.type === 'client'">
          <v-text-field v-model="form.creditLimit" label="Cupo de credito" type="number" min="0" variant="outlined" density="comfortable" />
        </v-col>
        <v-col cols="12" :md="props.type === 'client' ? 6 : 12">
          <v-text-field
            v-model="form.taxCondition"
            :label="props.type === 'client' ? 'Clasificacion tributaria' : 'Regimen o tratamiento fiscal'"
            variant="outlined"
            density="comfortable"
          />
        </v-col>
      </v-row>

      <v-text-field
        v-model="form.tagsText"
        label="Etiquetas"
        hint="Separalas por coma. Ejemplo: frecuente, credito, nacional"
        persistent-hint
        variant="outlined"
        density="comfortable"
      />

      <v-textarea v-model="form.notes" label="Notas internas" rows="3" auto-grow variant="outlined" density="comfortable" />
    </div>
  </v-form>
</template>
