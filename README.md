# ERP Producción

Sistema ERP (Enterprise Resource Planning) para gestión de producción manufacturera, desarrollado como aplicación de escritorio multiplataforma.

## Tecnologías

- **Frontend**: Vue 3 + TypeScript + Vite
- **UI Framework**: Vuetify (Material Design)
- **Estado**: Pinia
- **Enrutamiento**: Vue Router
- **Backend**: Rust + Tauri
- **Base de datos**: SQLite + SQLx

## Características

- Gestión completa de datos maestros (productos, materiales, recetas, unidades)
- Sistema de conversión de unidades
- Historial de costos de materiales
- Módulos de inventario y finanzas
- Dashboard con métricas
- Interfaz de producción

## Instalación y Desarrollo

### Prerrequisitos

- Node.js
- Rust
- Tauri CLI

### Instalación

```bash
npm install
```

### Desarrollo

```bash
# Frontend solo
npm run dev

# Con Tauri (aplicación completa)
npm run tauri:dev
```

### Build

```bash
npm run build
npm run tauri:build
```

## Estructura del Proyecto

- `src/`: Código fuente del frontend
- `src-tauri/`: Código fuente del backend Rust
- `docs/`: Documentación del proyecto
- `products/`: Datos de productos

## Documentación

Ver la documentación completa en la carpeta `docs/`:
- [Documentación del Backend](docs/backend/index.html)
- [Documentación del Frontend](docs/frontend/index.html)
