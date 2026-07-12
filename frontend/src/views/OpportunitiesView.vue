<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import AppCard from '../components/ui/AppCard.vue';
import AppTable from '../components/ui/AppTable.vue';
import OpportunityFilters from '../components/opportunities/OpportunityFilters.vue';
import OpportunityDetailDrawer from '../components/opportunities/OpportunityDetailDrawer.vue';
import OpportunityStatusBadge from '../components/opportunities/OpportunityStatusBadge.vue';
import AppPagination from '../components/ui/AppPagination.vue';
import { useOpportunitiesStore } from '../stores/opportunities.store';
import { getOpportunities } from '../services/opportunities.service';
import { useFormatters } from '../composables/useFormatters';

const store = useOpportunitiesStore();
const { formatUSD, formatPercent } = useFormatters();

const isLoading = ref(false);
const isDrawerOpen = ref(false);
const selectedOpportunity = ref<any>(null);

const currentPage = ref(1);
const totalRecords = ref(0);
const currentFilters = ref({});
const tableData = ref<any[]>([]);

const columns = [
  { key: 'id', label: 'ID' },
  { key: 'buy_exchange_name', label: 'Comprar' },
  { key: 'sell_exchange_name', label: 'Vender' },
  { key: 'profit_usd', label: 'Ganancia USD', align: 'right' as const },
  { key: 'profit_percent', label: 'Ganancia %', align: 'right' as const },
  { key: 'status', label: 'Estado', align: 'center' as const },
  { key: 'timestamp', label: 'Fecha' }
];

const data = computed(() => tableData.value);

const loadData = async (filters = {}) => {
  isLoading.value = true;
  try {
    const result = await getOpportunities({ ...filters, page: currentPage.value, limit: 10 });
    totalRecords.value = result.total_items || 0;
    tableData.value = result.data || result;
  } catch (error) {
    console.error('Error fetching opportunities:', error);
  } finally {
    isLoading.value = false;
  }
};

const handleFilter = (filters: any) => {
  currentFilters.value = filters;
  currentPage.value = 1;
  loadData(filters);
};

const handlePageChange = (page: number) => {
  currentPage.value = page;
  loadData(currentFilters.value);
};

const openDetail = (row: any) => {
  selectedOpportunity.value = row;
  isDrawerOpen.value = true;
};

onMounted(() => {
  loadData();
});

watch(() => store.summary.opportunities_count, (newVal, oldVal) => {
  if (newVal > oldVal && currentPage.value === 1) {
    // Reload silently when a new opportunity is detected and we are on page 1
    loadData(currentFilters.value);
  }
});
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>Oportunidades de Arbitraje</h2>
      <p class="text-muted">Historial completo de las oportunidades detectadas por el sistema.</p>
    </div>
    
    <OpportunityFilters @filter="handleFilter" />

    <AppCard>
      <AppTable :columns="columns" :data="data" :loading="isLoading" @row-click="openDetail">
        <template #cell-id="{ item }">
          <span class="text-muted text-xs">#{{ item.id?.toString().substring(0, 8) }}</span>
        </template>
         <template #cell-buy_exchange_name="{ item }">
          <span style="text-transform: capitalize;">{{ item.buy_exchange_name || item.buy_exchange }}</span>
        </template>
        <template #cell-sell_exchange_name="{ item }">
          <span style="text-transform: capitalize;">{{ item.sell_exchange_name || item.sell_exchange }}</span>
        </template>
        <template #cell-profit_usd="{ item }">
          <span class="numeric text-success">+{{ formatUSD(item.net_profit || 0) }}</span>
        </template>
        <template #cell-profit_percent="{ item }">
          <span class="numeric text-success">{{ formatPercent(item.net_profit_percent || 0) }}</span>
        </template>
        <template #cell-status="{ item }">
          <OpportunityStatusBadge :status="item.status || 'detected'" />
        </template>
        <template #cell-timestamp="{ item }">
          <span class="text-muted">{{ new Date(item.detected_at || item.timestamp).toLocaleString() }}</span>
        </template>
      </AppTable>
      <AppPagination 
        v-if="totalRecords > 0"
        :current-page="currentPage" 
        :total-items="totalRecords" 
        @page-change="handlePageChange" 
      />
    </AppCard>

    <OpportunityDetailDrawer 
      v-model="isDrawerOpen" 
      :opportunity="selectedOpportunity" 
    />
  </div>
</template>

<style scoped>
.view-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
}
.view-header h2 { margin: 0 0 8px 0; }
.view-header p { margin: 0; }
.text-xs { font-size: 12px; }
</style>
