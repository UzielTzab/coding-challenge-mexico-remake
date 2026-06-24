<script setup lang="ts">
import { ref } from 'vue';
import AppCard from '../components/ui/AppCard.vue';
import LogFilterTabs from '../components/logs/LogFilterTabs.vue';
import ServiceStatusCard from '../components/logs/ServiceStatusCard.vue';
import { useLogsStore } from '../stores/logs.store';
import { getLogs } from '../services/logs.service';
import { computed, onMounted } from 'vue';

const store = useLogsStore();
const activeTab = ref('all');

const logs = computed(() => {
  if (activeTab.value === 'all') return store.items;
  return store.items.filter(log => log.level === activeTab.value);
});

const services = [
  { name: 'Motor de Arbitraje', status: 'ok' as const, latency: 12 },
  { name: 'WebSocket Server', status: 'ok' as const, latency: 4 },
  { name: 'Base de Datos', status: 'ok' as const, latency: 25 },
  { name: 'Binance API', status: 'ok' as const, latency: 110 },
  { name: 'Kraken API', status: 'degraded' as const, latency: 850 }
];

const loadLogs = async () => {
  try {
    const data = await getLogs();
    const items = data.results || data;
    // Assuming store has a way to set all items or we just map them over
    store.items = items;
  } catch (error) {
    console.error('Error fetching logs:', error);
  }
};

onMounted(() => {
  loadLogs();
});
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>Consola del Sistema</h2>
      <p class="text-muted">Diagnóstico de servicios y logs en tiempo real.</p>
    </div>
    
    <div class="logs-layout">
      <div class="services-column">
        <h3 class="section-title">Estado de Servicios</h3>
        <div class="services-list">
          <ServiceStatusCard 
            v-for="srv in services" 
            :key="srv.name" 
            :service="srv.name" 
            :status="srv.status" 
            :latency="srv.latency" 
          />
        </div>
      </div>

      <div class="terminal-column">
        <AppCard class="terminal-card">
          <div class="terminal-header">
            <LogFilterTabs v-model="activeTab" />
          </div>
          <div class="terminal-body">
            <div v-for="log in logs" :key="log.id" class="log-entry">
              <span class="log-time text-muted">[{{ new Date(log.timestamp).toLocaleTimeString() }}]</span>
              <span class="log-type" :class="log.level">[{{ log.level.toUpperCase() }}]</span>
              <span class="log-msg">{{ log.message }}</span>
            </div>
          </div>
        </AppCard>
      </div>
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

.logs-layout {
  display: flex;
  gap: 24px;
  flex-grow: 1;
}

.services-column {
  width: 300px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section-title {
  margin: 0;
  font-size: 14px;
  text-transform: uppercase;
  color: var(--color-text-secondary);
  font-weight: 600;
  letter-spacing: 0.05em;
}

.services-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.terminal-column {
  flex-grow: 1;
  display: flex;
}

.terminal-card {
  width: 100%;
  display: flex;
  flex-direction: column;
  padding: 0;
  overflow: hidden;
}

.terminal-header {
  padding: 16px 24px 0 24px;
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
}

.terminal-body {
  flex-grow: 1;
  background: var(--color-bg-terminal);
  padding: 24px;
  overflow-y: auto;
  font-family: var(--font-mono);
  font-size: 13px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.log-entry {
  display: flex;
  gap: 12px;
}

.log-type {
  font-weight: 600;
}

.log-type.error { color: var(--color-danger); }
.log-type.system { color: var(--color-primary-light); }
.log-type.trading { color: var(--color-success); }
.log-msg { color: var(--color-text-primary); }
</style>
