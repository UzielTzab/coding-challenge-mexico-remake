<template>
  <div class="chart-wrapper-outer">
    <div class="chart-container" ref="chartContainer"></div>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted, shallowRef } from 'vue'
import { createChart, ColorType } from 'lightweight-charts'
import type { IChartApi, ISeriesApi, CandlestickData, Time } from 'lightweight-charts'
import { useMarketStore, type CandleData } from '../../stores/market'

const chartContainer = ref<HTMLElement | null>(null)
const chart = shallowRef<IChartApi | null>(null)
const candlestickSeries = shallowRef<ISeriesApi<"Candlestick"> | null>(null)

const marketStore = useMarketStore()
let unsubscribe: (() => void) | null = null

onMounted(() => {
  if (!chartContainer.value) return

  // Initialize Lightweight Charts with Navy Dark / Glassmorphism aesthetic
  chart.value = createChart(chartContainer.value, {
    layout: {
      background: { type: ColorType.Solid, color: 'transparent' }, // Transparent for glassmorphism integration
      textColor: '#d1d4dc',
    },
    grid: {
      vertLines: { color: 'rgba(42, 46, 57, 0.5)' },
      horzLines: { color: 'rgba(42, 46, 57, 0.5)' },
    },
    crosshair: {
      mode: 0, // Normal mode
    },
    timeScale: {
      borderColor: 'rgba(197, 203, 206, 0.8)',
      timeVisible: true,
      secondsVisible: false,
    },
    rightPriceScale: {
      borderColor: 'rgba(197, 203, 206, 0.8)',
    },
    autoSize: true, // Auto resize with container
  })

  // @ts-ignore - In some versions of lightweight-charts typing, addCandlestickSeries might require different generic or imports
  const series = (chart.value as any).addCandlestickSeries({
    upColor: '#26a69a',
    downColor: '#ef5350',
    borderVisible: false,
    wickUpColor: '#26a69a',
    wickDownColor: '#ef5350',
  })
  candlestickSeries.value = series

  // Set initial data if any (assuming empty for now, or fetch from REST later)
  series.setData([])

  // Subscribe to fast channel from Pinia store
  unsubscribe = marketStore.subscribe((candleData: CandleData) => {
    if (candlestickSeries.value) {
      // .update() avoids re-rendering the whole Vue tree, running at 60 FPS in canvas
      candlestickSeries.value.update(candleData as CandlestickData<Time>)
    }
  })
})

onUnmounted(() => {
  if (unsubscribe) {
    unsubscribe()
  }
  if (chart.value) {
    chart.value.remove()
  }
})
</script>

<style scoped>
.chart-wrapper-outer {
  flex: 1;
  width: 100%;
  height: 100%;
  min-height: 300px;
  border-radius: var(--radius-sm, 8px);
  overflow: hidden;
  position: relative;
  /* Glassmorphism/Navy Dark setup */
  background: rgba(13, 17, 23, 0.6);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.05);
}

.chart-container {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}
</style>
