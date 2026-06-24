<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted } from 'vue';
import { createChart } from 'lightweight-charts';
import type { IChartApi, ISeriesApi, Time } from 'lightweight-charts';
import AppCard from '../ui/AppCard.vue';

const props = defineProps<{
  data: { date: string; value: number; time: number }[];
  title?: string;
}>();

const chartContainer = ref<HTMLElement | null>(null);
let chart: IChartApi | null = null;
let lineSeries: ISeriesApi<"Area"> | null = null;

onMounted(() => {
  if (!chartContainer.value) return;

  chart = createChart(chartContainer.value, {
    layout: {
      background: { type: 'solid', color: 'transparent' },
      textColor: '#a0a0b0',
    },
    grid: {
      vertLines: { visible: false },
      horzLines: { color: 'rgba(255,255,255,0.05)' },
    },
    rightPriceScale: {
      borderVisible: false,
    },
    timeScale: {
      borderVisible: false,
      timeVisible: true,
      secondsVisible: true,
    },
    autoSize: true,
  });

  lineSeries = chart.addAreaSeries({
    lineColor: '#5e6ad2',
    topColor: 'rgba(94, 106, 210, 0.4)',
    bottomColor: 'rgba(94, 106, 210, 0.05)',
    lineWidth: 2,
    crosshairMarkerVisible: true,
    crosshairMarkerRadius: 4,
    crosshairMarkerBorderColor: '#fff',
    crosshairMarkerBackgroundColor: '#5e6ad2',
  });

  updateData();
});

watch(() => props.data, () => {
  updateData();
}, { deep: true });

const updateData = () => {
  if (!lineSeries || !props.data.length) return;
  
  // LightweightCharts requires unique, strictly ascending times
  const formattedData = props.data.map((d, index) => {
    // Lightweight charts needs unix timestamp in seconds (integer)
    const timeVal = Math.floor(d.time) + index; 
    return { 
      time: timeVal as Time, 
      value: d.value 
    };
  });
  
  lineSeries.setData(formattedData);
  chart?.timeScale().fitContent();
};

onUnmounted(() => {
  if (chart) {
    chart.remove();
  }
});
</script>

<template>
  <AppCard class="chart-card">
    <div class="chart-header">
      <span class="uppercase-label">{{ title || 'Evolución del P&L' }}</span>
    </div>
    <div class="chart-wrapper-outer">
      <div class="chart-container" ref="chartContainer"></div>
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

.chart-wrapper-outer {
  flex-grow: 1;
  position: relative;
  min-height: 0;
}

.chart-container {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}
</style>

