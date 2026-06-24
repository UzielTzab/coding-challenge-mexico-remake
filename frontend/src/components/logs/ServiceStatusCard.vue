<script setup lang="ts">
import AppCard from '../ui/AppCard.vue';

defineProps<{
  service: string;
  status: 'ok' | 'degraded' | 'down';
  latency?: number;
}>();
</script>

<template>
  <AppCard class="service-status">
    <div class="status-header">
      <span class="service-name">{{ service }}</span>
      <span class="status-dot" :class="status"></span>
    </div>
    <div class="status-details">
      <span class="status-text uppercase-label">{{ status === 'ok' ? 'Operativo' : status === 'degraded' ? 'Degradado' : 'Caído' }}</span>
      <span v-if="latency" class="latency numeric text-muted">{{ latency }}ms</span>
    </div>
  </AppCard>
</template>

<style scoped>
.service-status {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.service-name {
  font-weight: 500;
  color: var(--color-text-primary);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-dot.ok { background: var(--color-success); box-shadow: 0 0 8px rgba(16, 185, 129, 0.4); }
.status-dot.degraded { background: var(--color-warning); box-shadow: 0 0 8px rgba(245, 158, 11, 0.4); }
.status-dot.down { background: var(--color-danger); box-shadow: 0 0 8px rgba(239, 68, 68, 0.4); }

.status-details {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.latency {
  font-size: 12px;
}
</style>
