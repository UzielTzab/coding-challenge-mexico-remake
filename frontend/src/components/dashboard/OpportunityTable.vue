<script setup lang="ts">
import { computed, onMounted } from 'vue';
import AppCard from '../ui/AppCard.vue';
import { getOpportunities } from '../../services/opportunities.service';
import { useFormatters } from '../../composables/useFormatters';
import { useOpportunitiesStore } from '../../stores/opportunities.store';

const oppStore = useOpportunitiesStore();
const opportunities = computed(() => oppStore.items.slice(0, 5));
const { formatUSD, formatPercent } = useFormatters();

onMounted(async () => {
  try {
    const data = await getOpportunities();
    const results = data.data || data.results || data;
    if (Array.isArray(results) && oppStore.items.length === 0) {
      oppStore.items = results;
    }
  } catch (error) {
    console.error('Error fetching opportunities:', error);
  }
});
</script>

<template>
  <AppCard class="opp-card" variant="soft">
    <div class="opp-header">
      <h3>Oportunidades en Tiempo Real</h3>
    </div>
    <div class="table-container">
      <table class="opps-table">
        <thead>
          <tr>
            <th>Activo (Par)</th>
            <th>Comprar en</th>
            <th>Vender en</th>
            <th>Margen Bruto</th>
            <th>Ganancia Neta</th>
            <th>Estado</th>
          </tr>
        </thead>
        <tbody v-if="opportunities.length > 0">
          <tr v-for="opp in opportunities" :key="opp.id">
            <td>{{ opp.symbol || opp.pair || 'BTC/USDT' }}</td>
            <td style="text-transform: capitalize;">{{ opp.buy_exchange_name || opp.buy_exchange }}</td>
            <td style="text-transform: capitalize;">{{ opp.sell_exchange_name || opp.sell_exchange }}</td>
            <td :class="(opp.gross_spread_percent || 0) >= 0 ? 'text-success' : 'text-danger'">
              {{ formatPercent(opp.gross_spread_percent || 0) }}
            </td>
            <td :class="(opp.net_profit || opp.profit_usd || 0) >= 0 ? 'text-success' : 'text-danger'">
              {{ formatUSD(opp.net_profit || opp.profit_usd || 0) }}
            </td>
            <td>
              <span v-if="opp.status === 'profitable' || opp.status === 'executed'" class="status-badge success">EJECUTADA 🟢</span>
              <span v-else class="status-badge danger">DESCARTADA 🔴</span>
            </td>
          </tr>
        </tbody>
        <tbody v-else>
          <tr>
            <td colspan="6" class="empty-state">No hay oportunidades recientes</td>
          </tr>
        </tbody>
      </table>
    </div>
  </AppCard>
</template>

<style scoped>
.opp-card { display: flex; flex-direction: column; height: 100%; background-color: var(--color-bg-base); }
.opp-header { margin-bottom: 16px; }
.opp-header h3 { margin: 0; font-size: 16px; }
.table-container { overflow-x: auto; }
.opps-table { width: 100%; border-collapse: collapse; font-size: 13px; }
.opps-table th { text-align: left; padding: 12px 8px; color: var(--color-text-secondary); border-bottom: 1px solid var(--color-border); font-weight: 500; }
.opps-table td { padding: 12px 8px; border-bottom: 1px solid rgba(255, 255, 255, 0.05); }
.empty-state { text-align: center; padding: 40px; color: var(--color-text-muted); font-size: 14px; }

/* Status Badge */
.status-badge { font-weight: 600; font-size: 11px; padding: 4px 8px; border-radius: 4px; letter-spacing: 0.5px; }
.status-badge.success { background: rgba(16, 185, 129, 0.15); color: var(--color-success); }
.status-badge.danger { background: rgba(239, 68, 68, 0.15); color: var(--color-danger); }
</style>
