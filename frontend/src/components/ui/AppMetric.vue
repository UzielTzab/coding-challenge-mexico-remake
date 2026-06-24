<script setup lang="ts">
interface Props {
  label: string;
  value: string | number;
  variation?: number; // e.g., 5.4 for +5.4%
  prefix?: string;
  suffix?: string;
  isCurrency?: boolean;
}

defineProps<Props>();
</script>

<template>
  <div class="app-metric">
    <span class="metric-label">{{ label }}</span>
    <div class="metric-value-row">
      <span class="metric-value numeric">
        {{ prefix }}{{ value }}{{ suffix }}
      </span>
      <span 
        v-if="variation !== undefined" 
        class="metric-variation numeric"
        :class="{
          'text-success': variation > 0,
          'text-danger': variation < 0,
          'text-muted': variation === 0
        }"
      >
        <span v-if="variation > 0">▲</span>
        <span v-else-if="variation < 0">▼</span>
        {{ Math.abs(variation).toFixed(2) }}%
      </span>
    </div>
  </div>
</template>

<style scoped>
.app-metric {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.metric-label {
  font-size: var(--text-sm, 14px);
  color: var(--color-text-secondary);
}

.metric-value-row {
  display: flex;
  align-items:baseline;
  gap: 8px;
}

.metric-value {
  font-size: var(--text-xl, 24px);
  font-weight: 700;
  color: var(--color-text-primary);
}

.metric-variation {
  font-size: var(--text-sm, 14px);
  font-weight: 600;
}
</style>
