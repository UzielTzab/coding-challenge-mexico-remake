<script setup lang="ts">
import AppTabs from '../ui/AppTabs.vue';

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const tabs = [
  { id: 'all', label: 'Todos' },
  { id: 'system', label: 'Sistema' },
  { id: 'trading', label: 'Trading Engine' },
  { id: 'error', label: 'Errores' }
];

const updateTab = (id: string) => {
  emit('update:modelValue', id);
};
</script>

<template>
  <div class="log-filters">
    <AppTabs :tabs="tabs" :modelValue="modelValue" @update:modelValue="updateTab" />
  </div>
</template>

<style scoped>
.log-filters {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
