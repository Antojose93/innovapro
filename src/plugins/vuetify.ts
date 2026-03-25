import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const erpLightTheme = {
  dark: false,
  colors: {
    background: '#F8FAFC',
    surface: '#FFFFFF',
    primary: '#5568a5',
    secondary: '#475569',
    accent: '#38BDF8',
    error: '#EF4444',
    info: '#3B82F6',
    success: '#10B981',
    warning: '#F59E0B',
  },
}

export default createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: 'erpLightTheme',
    themes: {
      erpLightTheme,
    },
  },
  defaults: {
    VCard: {
      elevation: 1,
      rounded: 'lg',
    },
    VTextField: {
      variant: 'outlined',
      density: 'comfortable',
      color: 'primary',
    },
    VSelect: {
      variant: 'outlined',
      density: 'comfortable',
      color: 'primary',
    },
    VBtn: {
      style: 'text-transform: none;',
      rounded: 'md',
    },
    VDataTable: {
      density: 'comfortable',
    },
  },
})
