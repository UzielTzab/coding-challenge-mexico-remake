<script setup lang="ts">
interface Props {
  variant?: 'success' | 'danger' | 'warning' | 'info' | 'neutral';
}

withDefaults(defineProps<Props>(), {
  variant: 'neutral'
});
</script>

<template>
  <span :class="['badge', `badge--${variant}`]">
    <slot></slot>
  </span>
</template>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 24px;
  padding: 0 10px;
  border-radius: 999px;
  font-size: var(--text-xs, 12px);
  font-weight: 600;
}

.badge--success {
  background: rgba(3, 185, 135, 0.12);
  color: var(--color-success);
  border: 1px solid rgba(3, 185, 135, 0.30);
}

.badge--danger {
  background: rgba(250, 108, 110, 0.12);
  color: var(--color-danger);
  border: 1px solid rgba(250, 108, 110, 0.30);
}

.badge--warning {
  background: rgba(244, 183, 64, 0.12);
  color: var(--color-warning);
  border: 1px solid rgba(244, 183, 64, 0.30);
}

.badge--info {
  background: rgba(126, 176, 243, 0.12);
  color: var(--color-primary-light);
  border: 1px solid rgba(126, 176, 243, 0.30);
}

.badge--neutral {
  background: rgba(255, 255, 255, 0.05);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}
</style>
