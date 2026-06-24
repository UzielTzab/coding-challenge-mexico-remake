<script setup lang="ts">
import { computed } from 'vue';
import AppCard from '../ui/AppCard.vue';
import { Line } from 'vue-chartjs';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Filler,
  Legend
} from 'chart.js';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Filler,
  Legend
);

const props = defineProps<{
  data: any[];
  title?: string;
}>();

// Si no hay data real, usamos el mock provisional definido en el plan
const chartData = computed(() => {
  const hasData = props.data && props.data.length > 0;
  
  // Mock data temporal si no viene del backend
  const labels = hasData ? props.data.map(d => d.date || '') : ['Lun', 'Mar', 'Mie', 'Jue', 'Vie', 'Sab', 'Dom'];
  const dataPoints = hasData ? props.data.map(d => d.value || 0) : [120, 250, 200, 380, 310, 450, 520];

  return {
    labels,
    datasets: [
      {
        label: 'P&L USD',
        data: dataPoints,
        borderColor: '#5e6ad2', // var(--color-primary)
        backgroundColor: 'rgba(94, 106, 210, 0.1)',
        borderWidth: 2,
        tension: 0.4,
        fill: true,
        pointBackgroundColor: '#5e6ad2',
        pointBorderColor: '#fff',
        pointRadius: 4,
        pointHoverRadius: 6,
      }
    ]
  };
});

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: '#252836',
      titleColor: '#fff',
      bodyColor: '#a0a0b0',
      padding: 10,
      cornerRadius: 4,
      displayColors: false,
    }
  },
  scales: {
    x: {
      grid: { display: false, drawBorder: false },
      ticks: { color: '#a0a0b0', font: { size: 12 } }
    },
    y: {
      grid: { color: 'rgba(255,255,255,0.05)', drawBorder: false },
      ticks: { color: '#a0a0b0', font: { size: 12 } }
    }
  }
};
</script>

<template>
  <AppCard class="chart-card">
    <div class="chart-header">
      <span class="uppercase-label">{{ title || 'Evolución del P&L' }}</span>
    </div>
    <div class="chart-container">
      <Line :data="chartData" :options="chartOptions" />
    </div>
  </AppCard>
</template>

<style scoped>
.chart-card {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.chart-header {
  margin-bottom: 16px;
}

.chart-container {
  flex-grow: 1;
  min-height: 0;
  height: 250px; /* Estricto para evitar infinite resize */
  position: relative;
  width: 100%;
}
</style>
