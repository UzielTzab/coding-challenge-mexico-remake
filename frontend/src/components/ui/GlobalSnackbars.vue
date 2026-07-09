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
  top: 80px;
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
  color: #ffffff;
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0,0,0,0.25);
  font-size: 14px;
}

.snackbar-success { background: var(--color-success, #10b981); color: #ffffff; }
.snackbar-info { background: var(--color-success, #10b981); color: #ffffff; }
.snackbar-warning { background: var(--color-warning, #f59e0b); color: #ffffff; }
.snackbar-error { background: var(--color-danger, #ef4444); color: #ffffff; }
.snackbar-critical { 
  background: var(--color-danger, #ef4444); 
  color: #ffffff;
  font-weight: bold;
}

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
