<template>
  <div>
    <h1 class="text-h4 mb-6">Inventario</h1>
    <v-card>
      <v-card-text>
        <p class="text-h6">Control de inventario y almacenes</p>
        <v-list>
          <v-list-item prepend-icon="mdi-warehouse" title="Almacenes" />
          <v-list-item prepend-icon="mdi-package-variant" title="Stock actual" />
          <v-list-item prepend-icon="mdi-transfer" title="Movimientos" />
          <v-list-item prepend-icon="mdi-alert" title="Alertas de stock" />
        </v-list>
      </v-card-text>
    </v-card>
  </div>
</template>
