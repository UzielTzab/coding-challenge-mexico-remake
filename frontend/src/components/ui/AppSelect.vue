<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

interface Option {
  value: string | number;
  label: string;
}

interface Props {
  modelValue: string | number;
  label?: string;
  options: Option[];
  disabled?: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits(['update:modelValue']);

const isOpen = ref(false);
const selectRef = ref<HTMLElement | null>(null);

const selectedLabel = computed(() => {
  const selected = props.options.find(opt => opt.value === props.modelValue);
  return selected ? selected.label : 'Seleccionar...';
});

const toggleSelect = () => {
  if (!props.disabled) isOpen.value = !isOpen.value;
};

const selectOption = (value: string | number) => {
  emit('update:modelValue', value);
  isOpen.value = false;
};

// Close when clicking outside
const handleClickOutside = (event: MouseEvent) => {
  if (selectRef.value && !selectRef.value.contains(event.target as Node)) {
    isOpen.value = false;
  }
};

onMounted(() => document.addEventListener('click', handleClickOutside));
onUnmounted(() => document.removeEventListener('click', handleClickOutside));
</script>

<template>
  <div class="app-select-group" ref="selectRef">
    <label v-if="label" class="app-label">{{ label }}</label>
    <div 
      class="custom-select-trigger" 
      :class="{ 'is-open': isOpen, 'is-disabled': disabled }"
      @click="toggleSelect"
    >
      <span>{{ selectedLabel }}</span>
      <svg class="chevron" :class="{ 'rotate': isOpen }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
      </svg>
    </div>

    <!-- Dropdown Menu -->
    <div v-if="isOpen" class="custom-select-menu">
      <div 
        v-for="opt in options" 
        :key="opt.value" 
        class="custom-option"
        :class="{ 'is-selected': opt.value === modelValue }"
        @click="selectOption(opt.value)"
      >
        {{ opt.label }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.app-select-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 16px;
  position: relative;
}

.app-label {
  font-size: var(--text-sm, 14px);
  color: var(--color-text-secondary);
  font-weight: 500;
}

.custom-select-trigger {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 40px;
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm, 8px);
  padding: 0 12px;
  color: var(--color-text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 160ms ease;
  user-select: none;
}

.custom-select-trigger:hover:not(.is-disabled) {
  border-color: rgba(255, 255, 255, 0.1);
}

.custom-select-trigger.is-open {
  border-color: var(--color-primary-light, #7eb0f3);
  box-shadow: 0 0 0 2px rgba(126, 176, 243, 0.2);
}

.custom-select-trigger.is-disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.chevron {
  width: 16px;
  height: 16px;
  color: var(--color-text-muted);
  transition: transform 0.2s ease;
}

.chevron.rotate {
  transform: rotate(180deg);
}

.custom-select-menu {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--color-bg-elevated, #252836);
  border: 1px solid rgba(255, 255, 255, 0.08); /* Gris bajito minimalista */
  border-radius: var(--radius-sm, 8px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  z-index: 100;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.custom-option {
  padding: 10px 12px;
  font-size: 14px;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}

.custom-option:hover {
  background: rgba(255, 255, 255, 0.04);
  color: var(--color-text-primary);
}

.custom-option.is-selected {
  background: rgba(126, 176, 243, 0.1);
  color: var(--color-primary-light, #7eb0f3);
  font-weight: 500;
}
</style>
