<script setup lang="ts">
import AppCard from '../ui/AppCard.vue';
import TradeLegCard from './TradeLegCard.vue';
import { useFormatters } from '../../composables/useFormatters';

const props = defineProps<{
  operation: any;
}>();

const { formatUSD } = useFormatters();
</script>

<template>
  <div v-if="operation" class="operation-detail">
    <h3 class="detail-title">Detalle de Operación #{{ operation.id?.slice(0, 8) }}</h3>
    
    <div class="legs-container">
      <TradeLegCard 
        title="Tramo de Compra (Buy Leg)" 
        :exchange="operation.buy_exchange"
        :price="operation.buy_price"
        :amount="operation.amount"
        side="buy"
      />
      <div class="transfer-icon">
        <span class="material-symbols-outlined">sync_alt</span>
      </div>
      <TradeLegCard 
        title="Tramo de Venta (Sell Leg)" 
        :exchange="operation.sell_exchange"
        :price="operation.sell_price"
        :amount="operation.amount"
        side="sell"
      />
    </div>

    <AppCard class="summary-card" soft>
      <div class="summary-grid">
        <div class="summary-item">
          <span class="uppercase-label">Ganancia Bruta</span>
          <span class="numeric text-success">+{{ formatUSD(operation.gross_profit || 0) }}</span>
        </div>
        <div class="summary-item">
          <span class="uppercase-label">Comisiones Estimadas</span>
          <span class="numeric text-danger">-{{ formatUSD(operation.fees || 0) }}</span>
        </div>
        <div class="summary-item">
          <span class="uppercase-label">Ganancia Neta</span>
          <span class="numeric text-success net-profit">+{{ formatUSD(operation.net_profit || 0) }}</span>
        </div>
      </div>
    </AppCard>
  </div>
  <div v-else class="empty-state text-muted">
    Selecciona una operación de la tabla para ver sus detalles.
  </div>
</template>

<style scoped>
.operation-detail {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.detail-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: var(--color-text-primary);
}

.legs-container {
  display: flex;
  align-items: center;
  gap: 16px;
}

.transfer-icon {
  color: var(--color-text-muted);
  background: var(--color-bg-secondary);
  padding: 8px;
  border-radius: 50%;
  display: flex;
}

.summary-card {
  margin-top: 8px;
}

.summary-grid {
  display: flex;
  justify-content: space-between;
}

.summary-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.net-profit {
  font-size: 18px;
  font-weight: 600;
}

.empty-state {
  padding: 40px;
  text-align: center;
  background: var(--color-bg-secondary);
  border-radius: var(--radius-md);
}
</style>
