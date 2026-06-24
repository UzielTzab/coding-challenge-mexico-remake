<script setup lang="ts">
import { ref, onMounted } from 'vue';
import AppCard from '../components/ui/AppCard.vue';
import StrategySettingsForm from '../components/settings/StrategySettingsForm.vue';
import RiskSettingsForm from '../components/settings/RiskSettingsForm.vue';
import FeeSettingsTable from '../components/settings/FeeSettingsTable.vue';
import AppButton from '../components/ui/AppButton.vue';
import { getSettings, updateSettings } from '../services/settings.service';

const settings = ref({
  id: '1',
  min_spread_percent: 0.5,
  max_trade_usd: 1000,
  strategy_type: 'triangular',
  daily_loss_limit: 50,
  circuit_breaker_pause: 15,
  latency_alert_ms: 200,
  fees: [
    { exchange: 'binance', maker_fee: 0.001, taker_fee: 0.001, withdrawal_fee: 10 },
    { exchange: 'kraken', maker_fee: 0.0016, taker_fee: 0.0026, withdrawal_fee: 15 }
  ]
});

const isSaving = ref(false);

const loadSettings = async () => {
  try {
    const data = await getSettings();
    if (data) settings.value = { ...settings.value, ...data };
  } catch (error) {
    console.error('Error fetching settings:', error);
  }
};

const handleUpdate = (payload: any) => {
  settings.value = { ...settings.value, ...payload };
};

const saveChanges = async () => {
  isSaving.value = true;
  try {
    await updateSettings(settings.value.id, settings.value);
    // Mostrar notificación de éxito
  } catch (error) {
    console.error('Error saving settings:', error);
  } finally {
    isSaving.value = false;
  }
};

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <div class="title-section">
        <h2>Configuración del Motor</h2>
        <p class="text-muted">Ajusta los umbrales de riesgo, estrategias y perfiles de comisiones.</p>
      </div>
      <AppButton @click="saveChanges" :loading="isSaving">Guardar Cambios</AppButton>
    </div>
    
    <div class="settings-layout">
      <div class="main-settings">
        <AppCard class="settings-section">
          <h3>Estrategia y Ejecución</h3>
          <StrategySettingsForm :settings="settings" @update="handleUpdate" />
        </AppCard>

        <AppCard class="settings-section">
          <h3>Gestión de Riesgo</h3>
          <RiskSettingsForm :settings="settings" @update="handleUpdate" />
        </AppCard>
      </div>

      <div class="side-settings">
        <AppCard class="settings-section">
          <h3>Perfiles de Comisiones (Fees)</h3>
          <FeeSettingsTable :fees="settings.fees" />
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
}

.view-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.view-header h2 { margin: 0 0 8px 0; }
.view-header p { margin: 0; }

.settings-layout {
  display: flex;
  gap: 24px;
  align-items: flex-start;
}

.main-settings {
  flex: 2;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.side-settings {
  flex: 1;
}

.settings-section h3 {
  margin: 0 0 24px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  border-bottom: 1px solid var(--color-border);
  padding-bottom: 12px;
}
</style>
