<script setup lang="ts">
import AppCard from '../ui/AppCard.vue';
import AnimatedNumber from '../ui/AnimatedNumber.vue';

interface Props {
  title: string;
  value: string | number;
  variation?: number;
  prefix?: string;
  suffix?: string;
  subtextHtml?: string;
  decimals?: string | number;
}

defineProps<Props>();

const formatNumber = (val: number, props: Props) => {
  return val.toLocaleString('en-US', { 
    minimumFractionDigits: props?.decimals !== undefined ? Number(props.decimals) : 2, 
    maximumFractionDigits: props?.decimals !== undefined ? Number(props.decimals) : 2 
  });
};
</script>

<template>
  <AppCard class="kpi-card" variant="soft">
    <div class="kpi-label uppercase-label">{{ title }}</div>
    <div class="kpi-value-row">
      <span class="kpi-value numeric">
        {{ prefix }}<AnimatedNumber v-if="typeof value === 'number'" :value="value" :format="(v) => formatNumber(v, $props)" /><template v-else>{{ value }}</template>{{ suffix }}
      </span>
      <span v-if="variation" class="kpi-variation" :class="variation > 0 ? 'text-success' : 'text-danger'">
        {{ variation > 0 ? '▲' : '▼' }} {{ Math.abs(variation) }}%
      </span>
    </div>
    <div v-if="subtextHtml" class="kpi-subtext" v-html="subtextHtml"></div>
  </AppCard>
</template>

<style scoped>
.kpi-card {
  background: var(--color-bg-base);
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 20px 24px;
}
.kpi-label { margin-bottom: 4px; }
.kpi-value-row {
  display: flex;
  align-items: baseline;
  gap: 12px;
}
.kpi-value {
  font-size: 28px;
  font-weight: 500;
  color: var(--color-text-primary);
  letter-spacing: -0.02em;
  white-space: nowrap;
}
.kpi-variation {
  font-size: 13px;
  font-weight: 500;
  background: rgba(255, 255, 255, 0.05);
  padding: 4px 8px;
  border-radius: 6px;
}

.kpi-subtext {
  margin-top: 8px;
  font-size: 12px;
  color: var(--color-text-muted);
  display: flex;
  gap: 8px;
}

/* Scoped specifically so the HTML prop works */
:deep(.kpi-subtext .success) { color: var(--color-success); }
:deep(.kpi-subtext .danger) { color: var(--color-danger); }
</style>
