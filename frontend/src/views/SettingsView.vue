<script setup lang="ts">
import { ref, onMounted } from 'vue';
import AppCard from '../components/ui/AppCard.vue';
import StrategySettingsForm from '../components/settings/StrategySettingsForm.vue';
import RiskSettingsForm from '../components/settings/RiskSettingsForm.vue';
import FeeSettingsTable from '../components/settings/FeeSettingsTable.vue';
import AppSnackbar from '../components/ui/AppSnackbar.vue';
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
    { exchange: 'binance', maker_fee: 0.0001, taker_fee: 0.0001, withdrawal_fee: 10 },
    { exchange: 'kraken', maker_fee: 0.0001, taker_fee: 0.0001, withdrawal_fee: 15 }
  ]
});

const isSaving = ref(false);
const showSnackbar = ref(false);
const snackbarText = ref('');
const snackbarIcon = ref('info');
const snackbarIconColor = ref('var(--color-success)');

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
    snackbarText.value = 'Configuración guardada exitosamente';
    snackbarIcon.value = 'check_circle';
    snackbarIconColor.value = 'var(--color-success)';
    showSnackbar.value = true;
    setTimeout(() => { showSnackbar.value = false; }, 3000);
  } catch (error) {
    console.error('Error saving settings:', error);
    snackbarText.value = 'Error al guardar configuración';
    snackbarIcon.value = 'error';
    snackbarIconColor.value = 'var(--color-danger)';
    showSnackbar.value = true;
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
      <button class="btn-primary-light btn-save" :disabled="isSaving" @click="saveChanges">
        <div v-if="isSaving" class="btn-spinner"></div>
        <span v-else>Guardar Cambios</span>
      </button>
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

  <AppSnackbar
    v-model="showSnackbar"
    :text="snackbarText"
    :icon="snackbarIcon"
    :icon-color="snackbarIconColor"
    @close="showSnackbar = false"
  />
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

.btn-primary-light {
  background: var(--color-primary-light);
  color: #000000;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-weight: 600;
  font-size: 13px;
  cursor: pointer;
  transition: opacity 150ms;
}
.btn-primary-light:hover:not(:disabled) { 
  opacity: 0.85;
}
.btn-primary-light:disabled { opacity: 0.7; cursor: not-allowed; }

.btn-save {
  min-width: 130px;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 36px;
}

.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid rgba(0,0,0,0.3);
  border-top-color: #000;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

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
