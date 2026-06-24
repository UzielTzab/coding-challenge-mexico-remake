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
      {{ formatPercent(item.maker_fee) }}
    </template>
    <template #cell-taker_fee="{ item }">
      {{ formatPercent(item.taker_fee) }}
    </template>
    <template #cell-withdrawal_fee="{ item }">
      <span class="numeric">{{ item.withdrawal_fee }}</span>
    </template>
  </AppTable>
</template>
