<script setup lang="ts">
import { ref, watch } from 'vue';
import AppInput from '../ui/AppInput.vue';
import AppSelect from '../ui/AppSelect.vue';

const emit = defineEmits<{
  (e: 'filter', filters: any): void;
}>();

const statusOptions = [
  { label: 'Todos', value: '' },
  { label: 'Detectada', value: 'detected' },
  { label: 'Ejecutada', value: 'executed' },
  { label: 'Fallida', value: 'failed' }
];

const exchangeOptions = [
  { label: 'Todos', value: '' },
  { label: 'Binance', value: 'binance' },
  { label: 'Kraken', value: 'kraken' },
  { label: 'Coinbase', value: 'coinbase' }
];

const filters = ref({
  status: '',
  exchange: '',
  minProfit: ''
});

const applyFilters = () => {
  emit('filter', filters.value);
};

// Filter automatically when any filter changes
watch(filters, () => {
  applyFilters();
}, { deep: true });
</script>

<template>
  <div class="opportunity-filters">
    <div class="filter-group">
      <AppSelect 
        label="Estado" 
        v-model="filters.status" 
        :options="statusOptions" 
      />
      <AppSelect 
        label="Exchange" 
        v-model="filters.exchange" 
        :options="exchangeOptions" 
      />
      <AppInput 
        label="Min Profit (USD)" 
        v-model="filters.minProfit" 
        placeholder="Ej. 10.5" 
      />
    </div>
  </div>
</template>

<style scoped>
.opportunity-filters {
  display: flex;
  align-items: flex-end;
  gap: 16px;
  background: var(--color-bg-secondary);
  padding: 16px 24px;
  border-radius: var(--radius-md);
}

.filter-group {
  display: flex;
  gap: 16px;
  flex-grow: 1;
}

.filter-group > * {
  flex: 1;
  max-width: 200px;
}
</style>
