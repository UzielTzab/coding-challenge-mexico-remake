<script setup lang="ts">
interface Props {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
  type?: 'button' | 'submit' | 'reset';
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  variant: 'primary',
  type: 'button',
  disabled: false
});

defineEmits(['click']);
</script>

<template>
  <button 
    :class="['btn', `btn--${variant}`]"
    :type="type"
    :disabled="disabled"
    @click="$emit('click', $event)"
  >
    <slot></slot>
  </button>
</template>

<style scoped>
.btn {
  min-height: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 0 16px;
  border-radius: var(--radius-md, 12px);
  font-weight: 600;
  transition: 160ms ease;
  cursor: pointer;
  border: none;
  font-family: inherit;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn:not(:disabled):hover {
  transform: translateY(-1px);
}

.btn--primary {
  background: var(--gradient-primary);
  color: #FFFFFF;
  box-shadow: 0 8px 20px rgba(68, 81, 187, 0.35);
}

.btn--secondary {
  background: var(--color-bg-card-soft);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
}

.btn--danger {
  background: rgba(250, 108, 110, 0.14);
  color: var(--color-danger);
  border: 1px solid rgba(250, 108, 110, 0.35);
}

.btn--ghost {
  background: transparent;
  color: var(--color-text-secondary);
}

.btn--ghost:not(:disabled):hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--color-text-primary);
}
</style>
