<script setup lang="ts">
interface Props {
  variant?: 'default' | 'soft' | 'highlighted' | 'terminal';
}

withDefaults(defineProps<Props>(), {
  variant: 'default'
});
</script>

<template>
  <div :class="['app-card', `app-card--${variant}`]">
    <slot></slot>
  </div>
</template>

<style scoped>
.app-card {
  background: var(--color-bg-base);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg, 12px);
  padding: 24px;
}

.app-card--soft {
  background: var(--color-bg-secondary);
  border-color: transparent;
}

.app-card--highlighted {
  border-color: rgba(59, 130, 246, 0.2);
  box-shadow: 0 0 24px rgba(59, 130, 246, 0.05);
}

.app-card--terminal {
  background: var(--color-bg-terminal);
  font-family: var(--font-mono, monospace);
  border-color: transparent;
}
</style>
