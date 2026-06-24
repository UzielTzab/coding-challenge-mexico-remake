<script setup lang="ts">
interface Props {
  title?: string;
  message?: string;
  icon?: string;
}

withDefaults(defineProps<Props>(), {
  title: 'Sin resultados',
  message: 'No hay datos para mostrar en este momento.'
});
</script>

<template>
  <div class="app-empty-state">
    <div class="icon" v-if="icon">{{ icon }}</div>
    <h3>{{ title }}</h3>
    <p class="text-muted">{{ message }}</p>
    <slot></slot>
  </div>
</template>

<style scoped>
.app-empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 24px;
  text-align: center;
}

.icon {
  font-size: 48px;
  opacity: 0.5;
  margin-bottom: 16px;
}

h3 {
  margin: 0 0 8px 0;
  font-size: var(--text-md, 16px);
  color: var(--color-text-primary);
}

p {
  margin: 0;
  max-width: 400px;
  font-size: var(--text-sm, 14px);
}
</style>
