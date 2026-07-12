<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import AppCard from '../components/ui/AppCard.vue';
import OperationHistoryTable from '../components/operations/OperationHistoryTable.vue';
import AppPagination from '../components/ui/AppPagination.vue';
import { getTrades } from '../services/trades.service';
import { useOpportunitiesStore } from '../stores/opportunities.store';

const store = useOpportunitiesStore();

const isLoading = ref(false);
const operations = ref<any[]>([]);

const currentPage = ref(1);
const totalRecords = ref(0);

const loadData = async () => {
  isLoading.value = true;
  try {
    const result = await getTrades({ page: currentPage.value, limit: 10 });
    totalRecords.value = result.total_items || 0;
    operations.value = result.data || result;
  } catch (error) {
    console.error('Error fetching trades:', error);
  } finally {
    isLoading.value = false;
  }
};

const handlePageChange = (page: number) => {
  currentPage.value = page;
  loadData();
};

onMounted(() => {
  loadData();
});

watch(() => store.summary.trades_count, (newVal, oldVal) => {
  if (newVal > oldVal && currentPage.value === 1) {
    loadData();
  }
});
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>Operaciones Ejecutadas</h2>
      <p class="text-muted">Historial de órdenes de compra/venta enviadas a los exchanges.</p>
    </div>
    
    <div class="operations-layout">
      <AppCard class="history-section">
        <OperationHistoryTable 
          :operations="operations" 
          :isLoading="isLoading"
        />
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
  gap: 24px;
  height: 100%;
}
.view-header h2 { margin: 0 0 8px 0; }
.view-header p { margin: 0; }

.operations-layout {
  display: flex;
  gap: 24px;
  align-items: flex-start;
}

.history-section {
  flex: 1;
}
</style>
