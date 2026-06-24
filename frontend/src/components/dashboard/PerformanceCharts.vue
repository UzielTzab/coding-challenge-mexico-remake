<script setup lang="ts">
import { computed } from 'vue';
import { useOpportunitiesStore } from '../../stores/opportunities.store';
import PnlLineChart from '../performance/PnlLineChart.vue';

const oppStore = useOpportunitiesStore();

const chartData = computed(() => {
  if (oppStore.pnlHistory.length === 0) {
    // mock provisional initial
    return [
      { date: 'Inicio', value: oppStore.totalPnl, time: Math.floor(Date.now() / 1000) }
    ];
  }
  return oppStore.pnlHistory.map(h => ({ date: h.date, value: h.value, time: h.time }));
});
</script>

<template>
  <div class="charts-wrapper">
    <PnlLineChart :data="chartData" />
  </div>
</template>

<style scoped>
.charts-wrapper { height: 100%; min-height: 300px; display: flex; flex-direction: column; }
.charts-wrapper > * { flex-grow: 1; }
</style>
