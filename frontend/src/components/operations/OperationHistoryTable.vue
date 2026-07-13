<script setup lang="ts">
import AppTable from '../ui/AppTable.vue';
import { useFormatters } from '../../composables/useFormatters';

const props = defineProps<{
  operations: any[];
  isLoading?: boolean;
}>();

const emit = defineEmits<{
  (e: 'row-click', operation: any): void;
}>();

const { formatUSD, formatDate } = useFormatters();

const columns = [
  { key: 'id', label: 'ID Operación' },
  { key: 'symbol', label: 'Par' },
  { key: 'route', label: 'Ruta' },
  { key: 'quantity_btc', label: 'Volumen (BTC)' },
  { key: 'net_profit', label: 'Ganancia Neta' },
  { key: 'status', label: 'Estado' },
  { key: 'executed_at', label: 'Fecha / Hora' }
];

const handleRowClick = (row: any) => {
  emit('row-click', row);
};
</script>

<template>
  <div class="operation-history">
    <AppTable 
      :columns="columns" 
      :data="operations" 
      :loading="isLoading"
      @row-click="handleRowClick"
    >
      <template #cell-id="{ item }">
        <span class="text-muted">#{{ item.id }}</span>
      </template>
      <template #cell-symbol="{ item }">
        <strong>BTC/USDT</strong>
      </template>
      <template #cell-route="{ item }">
        <span style="text-transform: capitalize;">{{ item.buy_exchange }} → {{ item.sell_exchange }}</span>
      </template>
      <template #cell-quantity_btc="{ item }">
        <span class="numeric">{{ item.volume_btc }}</span>
      </template>
      <template #cell-net_profit="{ item }">
        <span :class="parseFloat(item.net_profit_usd || item.net_profit || '0') >= 0 ? 'text-success' : 'text-danger'">
          {{ formatUSD(parseFloat(item.net_profit_usd || item.net_profit || 0)) }}
        </span>
      </template>
      <template #cell-status="{ item }">
        <span :class="(item.execution_status || item.status) === 'executed' ? 'text-success' : 'text-danger'">
          {{ (item.execution_status || item.status || 'unknown').toUpperCase() }}
        </span>
      </template>
      <template #cell-executed_at="{ item }">
        <span class="text-muted">{{ formatDate(item.timestamp || item.executed_at) }}</span>
      </template>
    </AppTable>
  </div>
</template>

<style scoped>
.operation-history {
  width: 100%;
}
</style>
