<script setup lang="ts">
interface Props {
  modelValue: boolean;
  title: string;
  width?: string;
}

withDefaults(defineProps<Props>(), {
  width: '400px'
});

defineEmits(['update:modelValue', 'close']);

const close = (emit: any) => {
  emit('update:modelValue', false);
  emit('close');
};
</script>

<template>
  <Teleport to="body">
    <div class="drawer-overlay" v-if="modelValue" @click="close($emit)">
      <div class="drawer-content" :style="{ width }" @click.stop>
        <header class="drawer-header">
          <h3>{{ title }}</h3>
          <button class="close-btn" @click="close($emit)">&times;</button>
        </header>
        <div class="drawer-body">
          <slot></slot>
        </div>
        <footer class="drawer-footer" v-if="$slots.footer">
          <slot name="footer"></slot>
        </footer>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.drawer-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(12, 14, 31, 0.6);
  backdrop-filter: blur(2px);
  z-index: 900;
  display: flex;
  justify-content: flex-end;
}

.drawer-content {
  background: var(--color-bg-card);
  border-left: 1px solid var(--color-border);
  height: 100vh;
  display: flex;
  flex-direction: column;
  box-shadow: -8px 0 32px rgba(0, 0, 0, 0.2);
  animation: slideIn 300ms ease forwards;
}

@keyframes slideIn {
  from { transform: translateX(100%); }
  to { transform: translateX(0); }
}

.drawer-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.drawer-header h3 {
  margin: 0;
  font-size: var(--text-lg, 18px);
}

.close-btn {
  background: transparent;
  border: none;
  color: var(--color-text-secondary);
  font-size: 24px;
  cursor: pointer;
  line-height: 1;
}

.close-btn:hover {
  color: var(--color-text-primary);
}

.drawer-body {
  padding: 24px;
  overflow-y: auto;
  flex-grow: 1;
}

.drawer-footer {
  padding: 20px 24px;
  border-top: 1px solid var(--color-border);
}
</style>
