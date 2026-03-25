import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import type {
  CreateDirectoryEntryInput,
  DirectoryEntry,
  DirectoryEntityType,
  UpdateDirectoryEntryInput,
} from '@/types/directory'

const STORAGE_KEY = 'erp-directory-entries'

const seedEntries: DirectoryEntry[] = [
  {
    id: 1,
    type: 'client',
    status: 'active',
    displayName: 'Panaderia Central',
    segment: 'Mayorista',
    legalName: 'Panaderia Central SAS',
    contactName: 'Laura Diaz',
    documentType: 'NIT',
    documentNumber: '901234567',
    taxId: 'IVA responsable',
    phone: '+57 601 555 2001',
    mobile: '+57 300 555 1122',
    email: 'compras@panaderiacentral.com',
    secondaryEmail: 'pagos@panaderiacentral.com',
    website: null,
    country: 'Colombia',
    city: 'Bogota',
    address: 'Cra 18 # 72-14',
    paymentTerms: '30 dias',
    paymentMethod: 'Transferencia',
    creditLimit: 15000000,
    taxCondition: 'Gran contribuyente',
    bankReference: null,
    notes: 'Cliente con rotacion semanal y pedidos programados.',
    tags: ['frecuente', 'credito'],
    createdAt: '2026-03-10T09:00:00.000Z',
    updatedAt: '2026-03-22T14:30:00.000Z',
  },
  {
    id: 2,
    type: 'supplier',
    status: 'active',
    displayName: 'Molinos Andinos',
    segment: 'Harinas',
    legalName: 'Molinos Andinos LTDA',
    contactName: 'Carlos Mejia',
    documentType: 'NIT',
    documentNumber: '800456123',
    taxId: 'IVA responsable',
    phone: '+57 604 555 9988',
    mobile: '+57 311 444 8877',
    email: 'ventas@molinosandinos.com',
    secondaryEmail: 'cartera@molinosandinos.com',
    website: 'https://molinosandinos.com',
    country: 'Colombia',
    city: 'Medellin',
    address: 'Parque Industrial Norte, bodega 5',
    paymentTerms: '15 dias',
    paymentMethod: 'Transferencia',
    creditLimit: null,
    taxCondition: 'Regimen ordinario',
    bankReference: 'Bancolombia 019-455882-11',
    notes: 'Proveedor principal de harina panadera.',
    tags: ['estrategico', 'insumos'],
    createdAt: '2026-03-12T11:00:00.000Z',
    updatedAt: '2026-03-23T16:45:00.000Z',
  },
]

const normalizeText = (value?: string | null) => {
  const trimmed = value?.trim()
  return trimmed ? trimmed : null
}

const parseTags = (tags?: string[]) =>
  (tags ?? [])
    .map((tag) => tag.trim())
    .filter(Boolean)

const toEntry = (input: CreateDirectoryEntryInput, id: number, current?: DirectoryEntry): DirectoryEntry => {
  const now = new Date().toISOString()

  return {
    id,
    type: input.type,
    status: input.status,
    displayName: input.displayName.trim(),
    segment: input.segment.trim(),
    legalName: normalizeText(input.legalName),
    contactName: normalizeText(input.contactName),
    documentType: normalizeText(input.documentType),
    documentNumber: normalizeText(input.documentNumber),
    taxId: normalizeText(input.taxId),
    phone: normalizeText(input.phone),
    mobile: normalizeText(input.mobile),
    email: normalizeText(input.email),
    secondaryEmail: normalizeText(input.secondaryEmail),
    website: normalizeText(input.website),
    country: normalizeText(input.country),
    city: normalizeText(input.city),
    address: normalizeText(input.address),
    paymentTerms: normalizeText(input.paymentTerms),
    paymentMethod: normalizeText(input.paymentMethod),
    creditLimit: input.creditLimit ?? null,
    taxCondition: normalizeText(input.taxCondition),
    bankReference: normalizeText(input.bankReference),
    notes: normalizeText(input.notes),
    tags: parseTags(input.tags),
    createdAt: current?.createdAt ?? now,
    updatedAt: now,
  }
}

export const useDirectoryStore = defineStore('directory', () => {
  const entries = ref<DirectoryEntry[]>([])
  const loading = ref(false)
  const saving = ref(false)
  const error = ref<string | null>(null)
  const successMessage = ref<string | null>(null)

  const hasData = computed(() => entries.value.length > 0)
  const clients = computed(() => entries.value.filter((entry) => entry.type === 'client'))
  const suppliers = computed(() => entries.value.filter((entry) => entry.type === 'supplier'))

  const persist = () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(entries.value))
  }

  const hydrate = async () => {
    loading.value = true
    error.value = null

    try {
      const raw = localStorage.getItem(STORAGE_KEY)
      if (!raw) {
        entries.value = seedEntries
        persist()
        return
      }

      const parsed = JSON.parse(raw) as DirectoryEntry[]
      entries.value = Array.isArray(parsed) ? parsed : seedEntries
    } catch (hydrateError) {
      console.error('No fue posible cargar el directorio', hydrateError)
      entries.value = seedEntries
      persist()
      error.value = 'No fue posible leer el directorio guardado. Se restauraron datos base.'
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

  const setSuccess = (type: DirectoryEntityType, mode: 'create' | 'update' | 'delete') => {
    const label = type === 'client' ? 'Cliente' : 'Proveedor'
    const verb = mode === 'create' ? 'guardado' : mode === 'update' ? 'actualizado' : 'eliminado'
    successMessage.value = `${label} ${verb} correctamente.`
  }

  const createEntry = async (input: CreateDirectoryEntryInput) => {
    saving.value = true
    error.value = null

    try {
      const nextId = entries.value.reduce((maxId, entry) => Math.max(maxId, entry.id), 0) + 1
      entries.value = [toEntry(input, nextId), ...entries.value]
      persist()
      setSuccess(input.type, 'create')
    } catch (saveError) {
      console.error('No fue posible crear el registro', saveError)
      error.value = 'No fue posible guardar el registro en el directorio.'
      throw saveError
    } finally {
      saving.value = false
    }
  }

  const updateEntry = async (input: UpdateDirectoryEntryInput) => {
    saving.value = true
    error.value = null

    try {
      const current = entries.value.find((entry) => entry.id === input.id)
      if (!current) {
        throw new Error('Registro no encontrado')
      }

      entries.value = entries.value.map((entry) =>
        entry.id === input.id ? toEntry(input, input.id, current) : entry
      )
      persist()
      setSuccess(input.type, 'update')
    } catch (saveError) {
      console.error('No fue posible actualizar el registro', saveError)
      error.value = 'No fue posible actualizar el registro del directorio.'
      throw saveError
    } finally {
      saving.value = false
    }
  }

  const deleteEntry = async (id: number) => {
    saving.value = true
    error.value = null

    try {
      const removed = entries.value.find((entry) => entry.id === id)
      entries.value = entries.value.filter((entry) => entry.id !== id)
      persist()
      setSuccess(removed?.type ?? 'client', 'delete')
    } catch (deleteError) {
      console.error('No fue posible eliminar el registro', deleteError)
      error.value = 'No fue posible eliminar el registro del directorio.'
      throw deleteError
    } finally {
      saving.value = false
    }
  }

  return {
    entries,
    clients,
    suppliers,
    loading,
    saving,
    error,
    successMessage,
    hasData,
    hydrate,
    clearError,
    clearSuccess,
    createEntry,
    updateEntry,
    deleteEntry,
  }
})
