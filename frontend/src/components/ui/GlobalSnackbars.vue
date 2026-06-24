<script setup lang="ts">
import { useUiStore } from '../../stores/ui.store';

const uiStore = useUiStore();
</script>

<template>
  <div class="snackbar-container">
    <transition-group name="snackbar">
      <div 
        v-for="snackbar in uiStore.snackbars" 
        :key="snackbar.id" 
        class="snackbar" 
        :class="`snackbar-${snackbar.type}`"
      >
        <span class="snackbar-icon" v-if="snackbar.type === 'success'">🟢</span>
        <span class="snackbar-icon" v-if="snackbar.type === 'error'">🔴</span>
        <span class="snackbar-icon" v-if="snackbar.type === 'critical'">🚨</span>
        <span class="snackbar-icon" v-if="snackbar.type === 'info'">ℹ️</span>
        <span class="snackbar-icon" v-if="snackbar.type === 'warning'">⚠️</span>
        <span class="snackbar-text" v-html="snackbar.text"></span>
        <button class="snackbar-close" @click="uiStore.removeSnackbar(snackbar.id)">✕</button>
      </div>
    </transition-group>
  </div>
</template>

<style scoped>
.snackbar-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 12px;
  pointer-events: none;
}

.snackbar {
  pointer-events: auto;
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 280px;
  max-width: 450px;
  padding: 14px 18px;
  background: var(--color-bg-card, #202230);
  color: var(--color-text-primary, #ffffff);
  border-radius: 0 8px 8px 0; /* Contorno izquierdo recto */
  box-shadow: 0 4px 16px rgba(0,0,0,0.25);
  border-left: 4px solid var(--color-primary);
  font-size: 14px;
}

.snackbar-success { border-left-color: var(--color-success, #10b981); }
.snackbar-error { border-left-color: var(--color-danger, #ef4444); }
.snackbar-critical { 
  border-left-color: #ff0f0f;
  background: rgba(255, 15, 15, 0.15);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 15, 15, 0.5);
  box-shadow: 0 8px 32px rgba(255, 15, 15, 0.3);
  color: #fff;
  font-weight: bold;
}
.snackbar-warning { border-left-color: var(--color-warning, #f59e0b); }
.snackbar-info { border-left-color: var(--color-primary, #3b82f6); }

.snackbar-text {
  flex: 1;
  line-height: 1.4;
  font-weight: 500;
}

.snackbar-close {
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.5);
  cursor: pointer;
  padding: 4px;
  font-size: 14px;
}
.snackbar-close:hover {
  color: #fff;
}

.snackbar-enter-active,
.snackbar-leave-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}
.snackbar-enter-from {
  opacity: 0;
  transform: translateX(100%);
}
.snackbar-leave-to {
  opacity: 0;
  transform: translateY(15px);
}
</style>
