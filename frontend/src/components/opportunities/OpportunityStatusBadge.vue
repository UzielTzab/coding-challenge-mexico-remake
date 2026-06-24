<script setup lang="ts">
import AppBadge from '../ui/AppBadge.vue';
import { computed } from 'vue';

const props = defineProps<{
  status: string;
}>();

const variant = computed(() => {
  const map: Record<string, 'success' | 'danger' | 'warning' | 'info' | 'neutral'> = {
    'ejecutada': 'success',
    'executed': 'success',
    'descartada': 'danger',
    'failed': 'danger',
    'detectada': 'warning',
    'detected': 'warning'
  };
  return map[props.status.toLowerCase()] || 'neutral';
});

const label = computed(() => {
  return props.status.charAt(0).toUpperCase() + props.status.slice(1);
});
</script>

<template>
  <AppBadge :variant="variant">{{ label }}</AppBadge>
</template>
