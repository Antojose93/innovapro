export type DirectoryEntityType = 'client' | 'supplier'

export type DirectoryStatus = 'active' | 'prospect' | 'inactive' | 'blocked'

export interface DirectoryEntry {
  id: number
  type: DirectoryEntityType
  status: DirectoryStatus
  displayName: string
  segment: string
  legalName: string | null
  contactName: string | null
  documentType: string | null
  documentNumber: string | null
  taxId: string | null
  phone: string | null
  mobile: string | null
  email: string | null
  secondaryEmail: string | null
  website: string | null
  country: string | null
  city: string | null
  address: string | null
  paymentTerms: string | null
  paymentMethod: string | null
  creditLimit: number | null
  taxCondition: string | null
  bankReference: string | null
  notes: string | null
  tags: string[]
  createdAt: string
  updatedAt: string
}

export interface CreateDirectoryEntryInput {
  type: DirectoryEntityType
  status: DirectoryStatus
  displayName: string
  segment: string
  legalName?: string | null
  contactName?: string | null
  documentType?: string | null
  documentNumber?: string | null
  taxId?: string | null
  phone?: string | null
  mobile?: string | null
  email?: string | null
  secondaryEmail?: string | null
  website?: string | null
  country?: string | null
  city?: string | null
  address?: string | null
  paymentTerms?: string | null
  paymentMethod?: string | null
  creditLimit?: number | null
  taxCondition?: string | null
  bankReference?: string | null
  notes?: string | null
  tags?: string[]
}

export interface UpdateDirectoryEntryInput extends CreateDirectoryEntryInput {
  id: number
}
