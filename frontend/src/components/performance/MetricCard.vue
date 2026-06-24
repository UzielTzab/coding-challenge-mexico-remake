<script setup lang="ts">
import AppCard from '../ui/AppCard.vue';

defineProps<{
  title: string;
  value: string | number;
  trend?: 'up' | 'down' | 'neutral';
  trendValue?: string;
}>();
</script>

<template>
  <AppCard class="metric-card" soft>
    <span class="uppercase-label text-muted">{{ title }}</span>
    <div class="metric-body">
      <span class="metric-value numeric">{{ value }}</span>
      <div v-if="trendValue" class="trend" :class="trend">
        <span class="material-symbols-outlined icon">
          {{ trend === 'up' ? 'trending_up' : trend === 'down' ? 'trending_down' : 'trending_flat' }}
        </span>
        <span>{{ trendValue }}</span>
      </div>
    </div>
  </AppCard>
</template>

<style scoped>
.metric-card {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 20px;
}

.metric-body {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
}

.metric-value {
  font-size: 24px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.trend {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  font-weight: 500;
}

.trend.up { color: var(--color-success); }
.trend.down { color: var(--color-danger); }
.trend.neutral { color: var(--color-text-muted); }

.icon {
  font-size: 16px;
}
</style>
