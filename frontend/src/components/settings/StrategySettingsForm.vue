<script setup lang="ts">
import { ref, watch } from 'vue';
import AppInput from '../ui/AppInput.vue';
import AppSelect from '../ui/AppSelect.vue';

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
        label="Spread Mínimo (%)" 
        v-model="localSettings.min_spread_percent" 
        type="number"
      />
      <AppInput 
        label="Volumen Máximo por Trade (USD)" 
        v-model="localSettings.max_trade_usd" 
        type="number"
      />
      <AppSelect 
        label="Estrategia Principal" 
        v-model="localSettings.strategy_type" 
        :options="[{label:'Triangular', value:'triangular'}, {label:'Cross-Exchange', value:'cross_exchange'}]"
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
