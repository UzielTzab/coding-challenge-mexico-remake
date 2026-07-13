<script setup lang="ts">
import AppTable from '../ui/AppTable.vue';
import { useFormatters } from '../../composables/useFormatters';

const props = defineProps<{
  fees: any[];
}>();

const { formatPercent } = useFormatters();

const columns = [
  { key: 'exchange', label: 'Exchange' },
  { key: 'maker_fee', label: 'Maker Fee' },
  { key: 'taker_fee', label: 'Taker Fee' },
  { key: 'withdrawal_fee', label: 'Withdrawal Fee (USD)' }
];
</script>

<template>
  <AppTable :columns="columns" :data="fees">
    <template #cell-exchange="{ item }">
      <span style="text-transform: capitalize; font-weight: 500;">{{ item.exchange }}</span>
    </template>
    <template #cell-maker_fee="{ item }">
      <input type="number" step="0.0001" v-model.number="item.maker_fee" class="fee-input" />
    </template>
    <template #cell-taker_fee="{ item }">
      <input type="number" step="0.0001" v-model.number="item.taker_fee" class="fee-input" />
    </template>
    <template #cell-withdrawal_fee="{ item }">
      <input type="number" step="1" v-model.number="item.withdrawal_fee" class="fee-input" />
    </template>
  </AppTable>
</template>

<style scoped>
.fee-input {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  color: var(--color-text-primary);
  padding: 4px 8px;
  border-radius: 4px;
  width: 80px;
  font-family: monospace;
}
.fee-input:focus {
  outline: none;
  border-color: var(--color-primary);
}
</style>
