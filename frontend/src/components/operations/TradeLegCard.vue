<script setup lang="ts">
import AppCard from '../ui/AppCard.vue';
import { useFormatters } from '../../composables/useFormatters';

const props = defineProps<{
  title: string;
  exchange: string;
  price: number;
  amount: number;
  side: 'buy' | 'sell';
}>();

const { formatUSD } = useFormatters();
</script>

<template>
  <AppCard class="trade-leg-card">
    <div class="leg-header">
      <span class="uppercase-label">{{ title }}</span>
      <span class="exchange-badge">{{ exchange }}</span>
    </div>
    
    <div class="leg-details">
      <div class="detail-row">
        <span class="text-muted text-sm">Precio</span>
        <span class="numeric">{{ formatUSD(price) }}</span>
      </div>
      <div class="detail-row">
        <span class="text-muted text-sm">Cantidad</span>
        <span class="numeric">{{ amount }}</span>
      </div>
      <div class="detail-row total">
        <span class="text-muted text-sm">Total</span>
        <span class="numeric" :class="side === 'buy' ? 'text-danger' : 'text-success'">
          {{ side === 'buy' ? '-' : '+' }}{{ formatUSD(price * amount) }}
        </span>
      </div>
    </div>
  </AppCard>
</template>

<style scoped>
.trade-leg-card {
  flex: 1;
}

.leg-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--color-border);
}

.exchange-badge {
  background: var(--color-bg-secondary);
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
}

.leg-details {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.total {
  margin-top: 4px;
  padding-top: 12px;
  border-top: 1px solid var(--color-border);
  font-weight: 600;
}

.text-sm { font-size: 13px; }
</style>
