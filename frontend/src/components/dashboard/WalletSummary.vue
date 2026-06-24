<script setup lang="ts">
import { ref, onMounted } from 'vue';
import AppCard from '../ui/AppCard.vue';
import { getWallets } from '../../services/wallets.service';
import { useFormatters } from '../../composables/useFormatters';

const wallets = ref<any[]>([]);
const { formatUSD } = useFormatters();

onMounted(async () => {
  try {
    const data = await getWallets();
    const results = data.results || data;
    wallets.value = results.wallets || [];
  } catch (error) {
    console.error('Error fetching wallets:', error);
  }
});
</script>

<template>
  <AppCard class="wallet-summary">
    <h3>Saldos de Wallets</h3>
    <p class="text-muted" v-if="wallets.length === 0">No hay saldos registrados aún.</p>
    <div v-else class="wallet-list">
      <div v-for="w in wallets" :key="w.exchange" class="wallet-item">
        <span class="exchange-name">{{ w.exchange }}</span>
        <span class="wallet-balance">{{ formatUSD(w.totalUsdValue || 0) }}</span>
      </div>
    </div>
  </AppCard>
</template>

<style scoped>
.wallet-summary { height: 100%; display: flex; flex-direction: column; }
h3 { margin: 0 0 12px 0; font-size: 16px; }
.wallet-list { display: flex; flex-direction: column; gap: 8px; }
.wallet-item { display: flex; justify-content: space-between; padding: 8px; background: var(--color-bg-card-soft); border-radius: 6px; }
.exchange-name { font-weight: 500; text-transform: capitalize; }
.wallet-balance { color: var(--color-success); font-weight: 600; }
</style>
