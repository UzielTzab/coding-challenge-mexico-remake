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
  iconSvg?: string;
  iconBgColor?: string;
  iconColor?: string;
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
    <div class="kpi-header">
      <div class="kpi-label uppercase-label">{{ title }}</div>
    </div>
    <div class="kpi-value-row">
      <span class="kpi-value numeric">
        {{ prefix }}<AnimatedNumber v-if="typeof value === 'number'" :value="value" :format="(v) => formatNumber(v, $props)" /><template v-else>{{ value }}</template>{{ suffix }}
      </span>
    </div>
    <div class="kpi-footer">
      <span v-if="variation" class="kpi-variation" :class="variation > 0 ? 'text-success' : 'text-danger'">
        {{ variation > 0 ? '↗' : '↘' }} {{ Math.abs(variation) }}% <span class="variation-text">vs ayer</span>
      </span>
      <div v-if="subtextHtml" class="kpi-subtext" v-html="subtextHtml"></div>
    </div>
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
.kpi-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}


.kpi-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--color-text-secondary);
  letter-spacing: 0.05em;
  text-transform: uppercase;
}

.kpi-value-row {
  display: flex;
  align-items: baseline;
  gap: 12px;
  margin-top: 8px;
}
.kpi-value {
  font-size: 28px;
  font-weight: 500;
  color: var(--color-text-primary);
  letter-spacing: -0.02em;
  white-space: nowrap;
}

.kpi-footer {
  margin-top: auto;
  padding-top: 12px;
}

.kpi-variation {
  font-size: 12px;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 4px;
}
.kpi-variation.text-success { color: var(--color-success); }
.kpi-variation.text-danger { color: var(--color-danger); }

.variation-text {
  color: var(--color-text-muted);
  font-weight: 400;
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
