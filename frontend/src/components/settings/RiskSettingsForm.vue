<script setup lang="ts">
import { ref, watch } from 'vue';
import AppInput from '../ui/AppInput.vue';

const props = defineProps<{
  settings: any;
}>();

const emit = defineEmits<{
  (e: 'update', payload: any): void;
}>();

const localSettings = ref({ ...props.settings });

watch(() => props.settings, (newVal) => {
  localSettings.value = { ...newVal };
}, { deep: true });
</script>

<template>
  <div class="settings-form">
    <div class="form-grid">
      <AppInput 
        label="Límite Diario de Pérdidas (USD)" 
        v-model="localSettings.daily_loss_limit" 
        type="number"
      />
      <AppInput 
        label="Pausa de Circuit Breaker (minutos)" 
        v-model="localSettings.circuit_breaker_pause" 
        type="number"
      />
      <AppInput 
        label="Nivel de Alerta de Latencia (ms)" 
        v-model="localSettings.latency_alert_ms" 
        type="number"
      />
    </div>
  </div>
</template>

<style scoped>
.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 24px;
}
</style>
