<script setup lang="ts">
import AppCard from '../ui/AppCard.vue';
</script>

<template>
  <AppCard class="exec-panel" variant="soft">
    <h3>Panel de Ejecución</h3>
    <p class="text-muted">Selecciona una oportunidad para ver el desglose de ejecución simulada.</p>
  </AppCard>
</template>

<style scoped>
.exec-panel { height: 100%; }
h3 { margin: 0 0 12px 0; font-size: 16px; }
</style>
