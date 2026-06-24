<script setup lang="ts">
interface Props {
  modelValue: string | number;
  label?: string;
  type?: string;
  placeholder?: string;
  disabled?: boolean;
}

defineProps<Props>();
defineEmits(['update:modelValue']);
</script>

<template>
  <div class="app-input-group">
    <label v-if="label" class="app-label">{{ label }}</label>
    <input 
      :type="type || 'text'"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      class="app-input"
    />
  </div>
</template>

<style scoped>
.app-input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 16px;
}

.app-label {
  font-size: var(--text-sm, 14px);
  color: var(--color-text-secondary);
  font-weight: 500;
}

.app-input {
  height: 40px;
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm, 8px);
  padding: 0 12px;
  color: var(--color-text-primary);
  transition: 160ms ease;
}

.app-input:focus {
  outline: none;
  border-color: var(--color-primary-light);
  box-shadow: 0 0 0 2px rgba(126, 176, 243, 0.2);
}

.app-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.app-input::placeholder {
  color: var(--color-text-muted);
}
</style>
