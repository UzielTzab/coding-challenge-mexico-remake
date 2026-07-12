<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import AppCard from '../components/ui/AppCard.vue';
import WalletCard from '../components/wallets/WalletCard.vue';
import WalletMovementTable from '../components/wallets/WalletMovementTable.vue';
import { getWallets, getWalletMovements } from '../services/wallets.service';
import AppButton from '../components/ui/AppButton.vue';
import AppPagination from '../components/ui/AppPagination.vue';
import AppSkeleton from '../components/ui/AppSkeleton.vue';

const isLoading = ref(false);

const wallets = ref<any[]>([]);
const movements = ref<any[]>([]);

const currentPage = ref(1);
const totalRecords = ref(0);

const loadData = async () => {
  isLoading.value = true;
  try {
    const [walletsData, movementsData] = await Promise.all([
      getWallets(),
      getWalletMovements({ page: currentPage.value, limit: 50 })
    ]);
    
    const wResults = walletsData.results || walletsData;
    
    wallets.value = wResults.map((w: any) => ({
      exchange: w.exchange_name || w.exchange,
      balances: {
        'BTC': parseFloat(w.btc_available || '0'),
        'USDT': parseFloat(w.usdt_available || '0')
      },
      totalUsdValue: parseFloat(w.total_value_usd || '0')
    }));
    
    totalRecords.value = movementsData.count || 0;
    movements.value = movementsData.results || movementsData;
  } catch (error) {
    console.error('Error fetching wallets:', error);
  } finally {
    isLoading.value = false;
  }
};

const handlePageChange = (page: number) => {
  currentPage.value = page;
  loadData();
};

let intervalId: any;

onMounted(() => {
  loadData();
  intervalId = setInterval(() => {
    if (currentPage.value === 1 && !isLoading.value) {
      loadData();
    }
  }, 5000);
});

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId);
});
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <div class="header-content">
        <div>
          <h2>Saldos de Exchange</h2>
          <p class="text-muted">Gestión de portafolio y movimientos de capital.</p>
        </div>
        <AppButton variant="secondary" @click="loadData" :disabled="isLoading">
          <span class="material-symbols-outlined" :class="{ 'spin': isLoading }">refresh</span>
          Actualizar
        </AppButton>
      </div>
    </div>
    
    <div class="wallets-grid" v-if="isLoading && wallets.length === 0">
      <AppSkeleton v-for="i in 3" :key="i" height="200px" borderRadius="12px" />
    </div>
    <div class="wallets-grid" v-else>
      <WalletCard 
        v-for="w in wallets" 
        :key="w.exchange" 
        :exchange="w.exchange"
        :balances="w.balances"
        :totalUsdValue="w.totalUsdValue"
      />
    </div>

    <div class="movements-section">
      <h3>Historial de Movimientos</h3>
      <AppCard>
        <WalletMovementTable :movements="movements" :isLoading="isLoading" />
        <AppPagination 
          v-if="totalRecords > 0"
          :current-page="currentPage" 
          :total-items="totalRecords" 
          @page-change="handlePageChange" 
        />
      </AppCard>
    </div>
  </div>
</template>

<style scoped>
.view-container {
  display: flex;
  flex-direction: column;
  gap: 32px;
}
.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.view-header h2 { margin: 0 0 8px 0; }
.view-header p { margin: 0; }

.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.wallets-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
}

.movements-section h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  color: var(--color-text-primary);
}
</style>
