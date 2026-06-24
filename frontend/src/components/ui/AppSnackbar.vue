<script setup lang="ts">
defineProps<{
  modelValue: boolean;
  text: string;
  actionText?: string;
  icon?: string;
  iconColor?: string;
  loading?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
  (e: 'action'): void;
  (e: 'close'): void;
}>();

const handleAction = () => {
  emit('action');
};

const handleClose = () => {
  emit('update:modelValue', false);
  emit('close');
};
</script>

<template>
  <Transition name="snackbar-fade">
    <div v-if="modelValue" class="app-snackbar">
      <span class="material-symbols-outlined snackbar-icon" :style="{ color: iconColor || 'var(--color-warning)' }">
        {{ icon || 'info' }}
      </span>
      <span class="snackbar-text">{{ text }}</span>
      <button v-if="actionText" class="snackbar-btn" :disabled="loading" @click="handleAction">
        <div v-if="loading" class="btn-spinner"></div>
        <span v-else>{{ actionText }}</span>
      </button>
      <button class="snackbar-close" @click="handleClose">
        <span class="material-symbols-outlined">close</span>
      </button>
    </div>
  </Transition>
</template>

<style scoped>
.app-snackbar {
  position: fixed;
  bottom: 32px;
  right: 32px;
  background-color: var(--color-bg-card);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
  padding: 16px 24px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 16px;
  z-index: 9999;
  box-shadow: 0 4px 16px rgba(0,0,0,0.25);
}

.snackbar-fade-enter-active,
.snackbar-fade-leave-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.snackbar-fade-enter-from {
  transform: translateY(50px);
  opacity: 0;
}

.snackbar-fade-leave-to {
  transform: translateY(20px);
  opacity: 0;
}

.snackbar-text {
  font-size: 14px;
  font-weight: 500;
}

.snackbar-btn {
  background: var(--color-primary);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
  min-width: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.snackbar-btn:hover:not(:disabled) {
  background: #3a5bcf;
}

.snackbar-btn:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.snackbar-close {
  background: none;
  border: none;
  color: var(--color-text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  margin-left: 8px;
}
.snackbar-close:hover {
  color: var(--color-text-primary);
}
</style>
