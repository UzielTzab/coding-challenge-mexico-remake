<script setup lang="ts">
defineProps<{
  size?: 'sm' | 'md' | 'lg';
  text?: string;
}>();
</script>

<template>
  <div class="app-loader" :class="`size-${size || 'md'}`">
    <div class="spinner"></div>
    <span v-if="text" class="text-muted text-sm mt-2">{{ text }}</span>
  </div>
</template>

<style scoped>
.app-loader {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.spinner {
  border: 3px solid var(--color-border);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.size-sm .spinner { width: 16px; height: 16px; border-width: 2px; }
.size-md .spinner { width: 32px; height: 32px; border-width: 3px; }
.size-lg .spinner { width: 48px; height: 48px; border-width: 4px; }

@keyframes spin {
  to { transform: rotate(360deg); }
}

.mt-2 { margin-top: 8px; }
.text-sm { font-size: 13px; }
</style>
