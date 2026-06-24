<script setup lang="ts">
import AppCard from '../ui/AppCard.vue';
const props = defineProps<{
  exchange: string;
  balances: Record<string, number>;
}>();
</script>

<template>
  <AppCard class="wallet-card">
    <div class="wallet-header">
      <div class="exchange-title">
        <span class="material-symbols-outlined icon">account_balance_wallet</span>
        <h3>{{ exchange }}</h3>
      </div>
    </div>
    
    <div class="balances-list">
      <div v-for="(amount, asset) in balances" :key="asset" class="balance-item">
        <div class="asset-info">
          <div class="asset-icon">{{ asset.charAt(0) }}</div>
          <span class="asset-name">{{ asset }}</span>
        </div>
        <span class="numeric amount">{{ amount }}</span>
      </div>
    </div>
  </AppCard>
</template>

<style scoped>
.wallet-card {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.wallet-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--color-border);
}

.exchange-title {
  display: flex;
  align-items: center;
  gap: 8px;
}

.exchange-title h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  text-transform: capitalize;
}

.icon {
  color: var(--color-primary-light);
}

.balances-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.balance-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: var(--color-bg-secondary);
  border-radius: var(--radius-sm);
}

.asset-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.asset-icon {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: var(--color-primary);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
}

.asset-name {
  font-weight: 500;
}

.amount {
  font-family: var(--font-mono);
}
</style>
