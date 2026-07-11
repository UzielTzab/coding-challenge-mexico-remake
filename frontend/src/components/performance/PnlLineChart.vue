<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted } from 'vue';
import { createChart } from 'lightweight-charts';
import type { IChartApi, ISeriesApi, Time } from 'lightweight-charts';
import AppCard from '../ui/AppCard.vue';
import { useOpportunitiesStore } from '../../stores/opportunities.store';

const oppStore = useOpportunitiesStore();

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
    <div class="chart-footer-kpis">
      <div class="footer-kpi">
        <div class="kpi-icon" style="color: #6366F1; background: transparent">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm0-14c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zm0 10c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z"/></svg>
        </div>
        <div class="kpi-info">
          <div class="kpi-title">Oportunidades detectadas</div>
          <div class="kpi-val">{{ oppStore.summary.opportunities_count }}</div>
        </div>
      </div>
      <div class="footer-kpi">
        <div class="kpi-icon" style="color: #10B981; background: transparent">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
        </div>
        <div class="kpi-info">
          <div class="kpi-title">Oportunidades ejecutadas</div>
          <div class="kpi-val">{{ oppStore.summary.trades_count }} <span class="val-sub">({{ oppStore.summary.opportunities_count ? ((oppStore.summary.trades_count / oppStore.summary.opportunities_count) * 100).toFixed(1) : 0 }}%)</span></div>
        </div>
      </div>
      <div class="footer-kpi">
        <div class="kpi-icon" style="color: #3B82F6; background: transparent">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M16 6l2.29 2.29-4.88 4.88-4-4L2 16.59 3.41 18l6-6 4 4 6.3-6.29L22 12V6z"/></svg>
        </div>
        <div class="kpi-info">
          <div class="kpi-title">Volumen total</div>
          <div class="kpi-val">₿ {{ oppStore.summary.total_volume }}</div>
        </div>
      </div>
      <div class="footer-kpi">
        <div class="kpi-icon" style="color: #A855F7; background: transparent">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M11 2v20c-5.07-.5-9-4.79-9-10s3.93-9.5 9-10zm2 0v8.99L21.48 18c.84-1.74 1.34-3.71 1.34-5.83 0-5.18-4.22-9.42-9.42-9.49l-.4-.01V2zm0 10.99V22c5.07-.5 9-4.79 9-10H13v.99z"/></svg>
        </div>
        <div class="kpi-info">
          <div class="kpi-title">Spread promedio</div>
          <div class="kpi-val">{{ oppStore.summary.average_spread }}%</div>
        </div>
      </div>
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

.chart-footer-kpis {
  display: flex;
  justify-content: space-between;
  border-top: 1px solid var(--color-border);
  padding-top: 20px;
  margin-top: 20px;
}

.footer-kpi {
  display: flex;
  align-items: center;
  gap: 12px;
}

.kpi-icon {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.kpi-icon svg {
  width: 18px;
  height: 18px;
}

.kpi-info {
  display: flex;
  flex-direction: column;
}

.kpi-title {
  font-size: 11px;
  color: var(--color-text-muted);
}

.kpi-val {
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-primary);
  display: flex;
  align-items: baseline;
  gap: 6px;
}

.val-sub {
  font-size: 12px;
  color: var(--color-text-muted);
}
</style>

