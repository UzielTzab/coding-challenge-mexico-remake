<script setup lang="ts">
import AppDrawer from '../ui/AppDrawer.vue';
import OpportunityStatusBadge from './OpportunityStatusBadge.vue';
import { useFormatters } from '../../composables/useFormatters';

const props = defineProps<{
  modelValue: boolean;
  opportunity: any; // Ideally typed to Opportunity domain type
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();

const { formatUSD, formatPercent } = useFormatters();

const close = () => {
  emit('update:modelValue', false);
};
</script>

<template>
  <AppDrawer 
    :modelValue="modelValue" 
    @update:modelValue="close" 
    title="Detalle de Oportunidad"
  >
    <div v-if="opportunity" class="drawer-content">
      <div class="header-info">
        <div class="info-group">
          <span class="uppercase-label">ID Oportunidad</span>
          <span class="numeric">{{ opportunity.id }}</span>
        </div>
        <OpportunityStatusBadge :status="opportunity.status" />
      </div>

      <div class="metrics-grid">
        <div class="metric-item">
          <span class="uppercase-label">Comprar en</span>
          <span class="text-primary">{{ opportunity.buy_exchange }}</span>
        </div>
        <div class="metric-item">
          <span class="uppercase-label">Vender en</span>
          <span class="text-primary">{{ opportunity.sell_exchange }}</span>
        </div>
        <div class="metric-item">
          <span class="uppercase-label">Ganancia USD</span>
          <span class="numeric text-success">+{{ formatUSD(opportunity.profit_usd || 0) }}</span>
        </div>
        <div class="metric-item">
          <span class="uppercase-label">Spread</span>
          <span class="numeric">{{ formatPercent(opportunity.profit_percent || 0) }}</span>
        </div>
      </div>

      <div class="execution-log">
        <h4 class="uppercase-label">Registro de Ejecución</h4>
        <div class="log-box">
          <p class="text-muted text-sm">Simulando ejecución en {{ opportunity.buy_exchange }}...</p>
          <p class="text-muted text-sm">Validando liquidez de {{ opportunity.pair }}...</p>
          <p class="text-success text-sm">Orden completada.</p>
        </div>
      </div>
    </div>
    <div v-else class="drawer-loading">
      Cargando información...
    </div>
  </AppDrawer>
</template>

<style scoped>
.drawer-content {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.header-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--color-border);
}

.info-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.metrics-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}

.metric-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.metric-item span:last-child {
  font-size: 16px;
  font-weight: 500;
}

.execution-log {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.log-box {
  background: var(--color-bg-terminal);
  padding: 16px;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.text-sm { font-size: 13px; }
.drawer-loading { padding: 32px; text-align: center; color: var(--color-text-muted); }
</style>
