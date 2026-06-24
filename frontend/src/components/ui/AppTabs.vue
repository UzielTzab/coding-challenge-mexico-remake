<script setup lang="ts">

const props = defineProps<{
  tabs: { id: string; label: string }[];
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const selectTab = (id: string) => {
  emit('update:modelValue', id);
};
</script>

<template>
  <div class="app-tabs">
    <button 
      v-for="tab in tabs" 
      :key="tab.id"
      class="tab-button"
      :class="{ 'active': modelValue === tab.id }"
      @click="selectTab(tab.id)"
    >
      {{ tab.label }}
    </button>
  </div>
</template>

<style scoped>
.app-tabs {
  display: flex;
  gap: 8px;
  border-bottom: 1px solid var(--color-border);
  margin-bottom: 16px;
}

.tab-button {
  background: transparent;
  border: none;
  padding: 10px 16px;
  color: var(--color-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 150ms ease;
}

.tab-button:hover {
  color: var(--color-text-primary);
}

.tab-button.active {
  color: var(--color-primary-light);
  border-bottom-color: var(--color-primary-light);
}
</style>
