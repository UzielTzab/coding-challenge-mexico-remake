<script setup lang="ts">
import { computed } from 'vue';
import AppCard from '../ui/AppCard.vue';
import { Doughnut } from 'vue-chartjs';
import {
  Chart as ChartJS,
  ArcElement,
  Tooltip,
  Legend
} from 'chart.js';

ChartJS.register(ArcElement, Tooltip, Legend);

const props = defineProps<{
  stats: {
    executed: number;
    failed: number;
    discarded: number;
  };
}>();

const total = computed(() => props.stats.executed + props.stats.failed + props.stats.discarded);
const winRate = computed(() => total.value > 0 ? Math.round((props.stats.executed / total.value) * 100) : 0);

const chartData = computed(() => ({
  labels: ['Ejecutadas', 'Fallidas', 'Descartadas'],
  datasets: [
    {
      data: [props.stats.executed, props.stats.failed, props.stats.discarded],
      backgroundColor: ['#10b981', '#ef4444', '#f59e0b'],
      borderWidth: 0,
      hoverOffset: 4
    }
  ]
}));

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  cutout: '75%',
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: '#252836',
      titleColor: '#fff',
      bodyColor: '#a0a0b0',
      padding: 10,
      cornerRadius: 4,
      displayColors: true,
    }
  }
};
</script>

<template>
  <AppCard class="chart-card">
    <div class="chart-header">
      <span class="uppercase-label">Tasa de Éxito</span>
    </div>
    <div class="donut-container">
      <div class="donut-wrapper">
        <Doughnut :data="chartData" :options="chartOptions" />
        <div class="donut-center">
          <span class="win-rate">{{ winRate }}%</span>
          <span class="text-muted text-sm">Win Rate</span>
        </div>
      </div>
      <div class="legend">
        <div class="legend-item">
          <span class="dot success"></span>
          <span>Ejecutadas ({{ stats.executed }})</span>
        </div>
        <div class="legend-item">
          <span class="dot danger"></span>
          <span>Fallidas ({{ stats.failed }})</span>
        </div>
        <div class="legend-item">
          <span class="dot warning"></span>
          <span>Descartadas ({{ stats.discarded }})</span>
        </div>
      </div>
    </div>
  </AppCard>
</template>

<style scoped>
.chart-card {
  height: 100%;
}

.chart-header {
  margin-bottom: 24px;
}

.donut-container {
  display: flex;
  align-items: center;
  justify-content: space-around;
  gap: 24px;
}

.donut-wrapper {
  position: relative;
  width: 140px;
  height: 140px;
  flex-shrink: 0;
}

.donut-center {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  pointer-events: none;
}

.win-rate {
  font-size: 24px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.legend {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.dot.success { background: var(--color-success); }
.dot.danger { background: var(--color-danger); }
.dot.warning { background: var(--color-warning); }

.text-sm { font-size: 12px; }
</style>
