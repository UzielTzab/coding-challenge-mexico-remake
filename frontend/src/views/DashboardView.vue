<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { getDashboardSummary, getExchanges } from '../services/dashboard.service';
import { useMarketStore } from '../stores/market.store';
import { useOpportunitiesStore } from '../stores/opportunities.store';
import KpiCard from '../components/dashboard/KpiCard.vue';
import ExchangeCard from '../components/dashboard/ExchangeCard.vue';
import OpportunityTable from '../components/dashboard/OpportunityTable.vue';
import PerformanceCharts from '../components/dashboard/PerformanceCharts.vue';
import AppSkeleton from '../components/ui/AppSkeleton.vue';

const marketStore = useMarketStore();
const oppStore = useOpportunitiesStore();
const isLoading = ref(true);

const averageBtcPrice = computed(() => {
  const snaps = Object.values(marketStore.snapshots);
  if (snaps.length === 0) return 0;
  const sum = snaps.reduce((acc, s) => acc + ((s.bid + s.ask) / 2), 0);
  return sum / snaps.length;
});

onMounted(async () => {
  try {
    const data = await getDashboardSummary();
    let result = (data as any).results ? (data as any).results : data;
    if (Array.isArray(result) && result.length > 0) result = result[0];
    
    if (result) {
      oppStore.setSummary({
        global_win_rate: parseFloat(result.win_rate_percent) || 0,
        trades_count: result.total_trades || 0,
        discarded_opportunities: result.discarded_opportunities || 0,
        opportunities_count: (result.total_trades || 0) + (result.discarded_opportunities || 0),
        average_cost: parseFloat(result.total_fees_usd) || 0
      });
      oppStore.setInitialPnl(parseFloat(result.total_pnl_usd) || 0);
    }
  } catch (error) {
    console.error('Error loading dashboard summary', error);
  }

  try {
    const exData = await getExchanges();
    const exchanges = (exData as any).results || exData;
    if (Array.isArray(exchanges)) {
      exchanges.forEach((ex: any) => {
        marketStore.upsertSnapshot({
          exchange: ex.name ? ex.name.toLowerCase() : ex.exchange,
          pair: ex.pair || 'BTC/USDT',
          bid: ex.last_bid || ex.bid || 0,
          ask: ex.last_ask || ex.ask || 0,
          timestamp: new Date().toISOString()
        });
      });
    }
  } catch (error) {
    console.error('Error loading exchanges:', error);
  } finally {
    isLoading.value = false;
  }
});
</script>

<template>
  <div class="dashboard-grid">
    <template v-if="isLoading">
      <div v-for="i in 6" :key="`kpi-${i}`" class="col-span-2">
        <AppSkeleton height="100px" borderRadius="12px" />
      </div>
      <div v-for="i in 3" :key="`ex-${i}`" class="col-span-4">
        <AppSkeleton height="220px" borderRadius="12px" />
      </div>
      <div class="col-span-7"><AppSkeleton height="350px" borderRadius="12px" /></div>
      <div class="col-span-5"><AppSkeleton height="350px" borderRadius="12px" /></div>
      <div class="col-span-5"><AppSkeleton height="300px" borderRadius="12px" /></div>
      <div class="col-span-7"><AppSkeleton height="300px" borderRadius="12px" /></div>
      <div class="col-span-12"><AppSkeleton height="250px" borderRadius="12px" /></div>
    </template>
    
    <template v-else>
      <!-- Fila 1: KPIs (5 cards) -->
      <KpiCard class="col-span-3" title="P&L Total" :value="oppStore.totalPnl" prefix="$" :variation="12.4"
        iconBgColor="transparent" iconColor="#9370DB"
        iconSvg='<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/></svg>'
      />
      <KpiCard class="col-span-2" title="Win Rate" :value="oppStore.summary.global_win_rate" suffix="%" :variation="0.01"
        iconBgColor="transparent" iconColor="#10B981"
        iconSvg='<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M7.5 11C9.43 11 11 9.43 11 7.5S9.43 4 7.5 4 4 5.57 4 7.5 5.57 11 7.5 11zm0-5C8.33 6 9 6.67 9 7.5S8.33 9 7.5 9 6 8.33 6 7.5 6.67 6 7.5 6zM16.5 13c-1.93 0-3.5 1.57-3.5 3.5s1.57 3.5 3.5 3.5 3.5-1.57 3.5-3.5-1.57-3.5-3.5-3.5zm0 5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"/><path d="M5.41 20L20 5.41 18.59 4 4 18.59z"/></svg>'
      />
      <KpiCard class="col-span-2" title="Ops Ejecutadas" :value="oppStore.summary.trades_count" decimals="0" :variation="8"
        iconBgColor="transparent" iconColor="#3B82F6"
        iconSvg='<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M4 10.5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm0-6c-.83 0-1.5.67-1.5 1.5S3.17 7.5 4 7.5 5.5 6.83 5.5 6 4.83 4.5 4 4.5zm0 12c-.83 0-1.5.68-1.5 1.5s.68 1.5 1.5 1.5 1.5-.68 1.5-1.5-.67-1.5-1.5-1.5zM7 19h14v-2H7v2zm0-6h14v-2H7v2zm0-8v2h14V5H7z"/></svg>'
      />
      <KpiCard class="col-span-2" title="Costo Promedio" :value="oppStore.summary.average_cost" prefix="$" :variation="-3.2"
        iconBgColor="transparent" iconColor="#F59E0B"
        iconSvg='<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 4.02 2 6.5s4.48 4.5 10 4.5 10-2.02 10-4.5S17.52 2 12 2zm0 7c-4.32 0-8-1.42-8-3.11V7.5c0 1.69 3.68 3.11 8 3.11s8-1.42 8-3.11V5.89C20 7.58 16.32 9 12 9zm0 4.5c-4.32 0-8-1.42-8-3.11v1.61c0 1.69 3.68 3.11 8 3.11s8-1.42 8-3.11v-1.61c0 1.69-3.68 3.11-8 3.11z"/><path d="M12 18c-4.32 0-8-1.42-8-3.11v1.61C4 18.19 7.68 19.61 12 19.61s8-1.42 8-3.11v-1.61c0 1.69-3.68 3.11-8 3.11z"/></svg>'
      />
      <KpiCard class="col-span-3" title="Precio Promedio BTC" :value="averageBtcPrice" prefix="$" :variation="1.1"
        iconBgColor="transparent" iconColor="#A855F7"
        iconSvg='<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M17.06 11.57c.59-.62.94-1.43.94-2.32 0-1.78-1.39-3.28-3.23-3.64V3h-2v2.16c-.53-.08-1.09-.12-1.67-.12V3h-2v2h-4v2h1.5c.83 0 1.5.67 1.5 1.5v9c0 .83-.67 1.5-1.5 1.5H5v2h4v2h2v-2h1.67c.58 0 1.14-.04 1.67-.12V21h2v-2.61c2.14-.29 3.82-2.12 3.82-4.39 0-1.12-.4-2.15-1.08-2.93zM10 7.5h2.5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5H10v-3zm3.5 9H10v-3.5h3.5c.96 0 1.75.78 1.75 1.75S14.46 16.5 13.5 16.5z"/></svg>'
      />

      <!-- Fila 2: Gráficas y Exchanges -->
      <PerformanceCharts class="col-span-8" />
      <div class="col-span-4 exchange-cards-container">
        <ExchangeCard 
          exchangeName="Binance" 
          :connected="Object.keys(marketStore.snapshots).some(k => k.includes('binance'))" 
          :marketData="Object.values(marketStore.snapshots).find(m => m.exchange === 'binance')" 
        />
        <ExchangeCard 
          exchangeName="Kraken" 
          :connected="Object.keys(marketStore.snapshots).some(k => k.includes('kraken'))" 
          :marketData="Object.values(marketStore.snapshots).find(m => m.exchange === 'kraken')" 
        />
      </div>

      <!-- Fila 3: Oportunidades (Full Width) -->
      <OpportunityTable class="col-span-12" />
    </template>
  </div>
</template>

<style scoped>
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: 16px;
  padding-bottom: 24px;
}

.col-span-2 { grid-column: span 2; }
.col-span-3 { grid-column: span 3; }
.col-span-4 { grid-column: span 4; }
.col-span-5 { grid-column: span 5; }
.col-span-6 { grid-column: span 6; }
.col-span-7 { grid-column: span 7; }
.col-span-8 { grid-column: span 8; }
.col-span-12 { grid-column: span 12; }

.exchange-cards-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

@media (max-width: 1200px) {
  .dashboard-grid { grid-template-columns: repeat(6, 1fr); }
  .col-span-2 { grid-column: span 2; }
  .col-span-3 { grid-column: span 3; }
  .col-span-4 { grid-column: span 6; }
  .col-span-5 { grid-column: span 6; }
  .col-span-6 { grid-column: span 6; }
  .col-span-7 { grid-column: span 6; }
  .col-span-8 { grid-column: span 6; }
}

@media (max-width: 768px) {
  .dashboard-grid { grid-template-columns: 1fr; }
  .col-span-2, .col-span-3, .col-span-4, .col-span-5, .col-span-7, .col-span-8, .col-span-12 {
    grid-column: span 1;
  }
}
</style>
