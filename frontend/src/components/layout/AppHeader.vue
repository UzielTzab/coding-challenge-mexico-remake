<script setup lang="ts">
import { useRoute } from 'vue-router';
import { computed, ref, onMounted } from 'vue';
import { getSettings, updateSettings } from '../../services/settings.service';
import { useBotStore } from '../../stores/bot.store';
import AppSnackbar from '../ui/AppSnackbar.vue';
import AppTooltip from '../ui/AppTooltip.vue';

const route = useRoute();
const botStore = useBotStore();
const settingId = ref<string | null>(null);
const backendStatus = ref(true);
const wsStatus = ref(true); // Se asume verdadero al inicio si no hay fallos en consola

const pageTitle = computed(() => {
  const map: Record<string, string> = {
    '/': 'Overview',
    '/markets': 'Mercados',
    '/opportunities': 'Oportunidades',
    '/operations': 'Operaciones',
    '/wallets': 'Saldos de Exchange',
    '/performance': 'Rendimiento',
    '/logs': 'Consola del Sistema',
    '/settings': 'Configuración'
  };
  return map[route.path] || 'Dashboard';
});

const isToggling = ref(false);
const showSnackbar = ref(false);
const snackbarText = ref('Bot detenido — Pulsa "Iniciar Bot" para comenzar el monitoreo en tiempo real');
const snackbarIcon = ref('info');
const snackbarIconColor = ref('var(--color-danger)');
const snackbarActionText = ref('INICIAR');

const showInitialSnackbar = () => {
  snackbarText.value = 'Bot detenido — Pulsa "Iniciar Bot" para comenzar el monitoreo en tiempo real';
  snackbarIcon.value = 'info';
  snackbarIconColor.value = 'var(--color-danger)';
  snackbarActionText.value = 'INICIAR';
  showSnackbar.value = true;
};

const loadSettings = async () => {
  try {
    const data = await getSettings();
    const results = data.results || data;
    if (results && results.length > 0) {
      settingId.value = results[0].id;
      const isRunning = results[0].is_running;
      botStore.setStatus(isRunning ? 'running' : 'stopped');
      backendStatus.value = true;
      
      if (!isRunning) {
        setTimeout(showInitialSnackbar, 500);
      }
    }
  } catch (error) {
    console.error('Error fetching settings for bot toggle', error);
    backendStatus.value = false;
    wsStatus.value = false;
    setTimeout(showInitialSnackbar, 500);
  }
};

const toggleBot = async (state: boolean) => {
  if (!settingId.value || isToggling.value) return;
  isToggling.value = true;
  
  if (showSnackbar.value) {
    showSnackbar.value = false; // Hide current snackbar while processing
  }

  try {
    if (state) {
      botStore.setStatus('starting');
      await new Promise(resolve => setTimeout(resolve, 1500));
    }
    await updateSettings(settingId.value, { is_running: state });
    botStore.setStatus(state ? 'running' : 'stopped');
    backendStatus.value = true;
    
    // Show success snackbar
    setTimeout(() => {
      snackbarText.value = state ? 'Bot iniciado con éxito. Conectado a exchanges.' : 'Bot detenido correctamente.';
      snackbarIcon.value = 'check_circle';
      snackbarIconColor.value = 'var(--color-success)';
      snackbarActionText.value = '';
      showSnackbar.value = true;
      
      // Auto-hide success message
      setTimeout(() => { showSnackbar.value = false; }, 3000);
    }, 100);

  } catch (error) {
    console.error('Error toggling bot', error);
    backendStatus.value = false;
    botStore.setStatus('stopped');
    
    // Show error snackbar
    setTimeout(() => {
      snackbarText.value = 'Error al iniciar el bot. Verifica tu conexión.';
      snackbarIcon.value = 'error';
      snackbarIconColor.value = 'var(--color-danger)';
      snackbarActionText.value = 'REINTENTAR';
      showSnackbar.value = true;
    }, 100);
  } finally {
    isToggling.value = false;
  }
};

const closeSnackbar = () => {
  showSnackbar.value = false;
};

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <header class="header">
    <div class="header-title">
      <h1>{{ pageTitle }}</h1>
    </div>
    
    <div class="header-actions">
      <div class="status-indicators">
        <div class="status">
          <span class="material-symbols-outlined status-icon" :class="backendStatus ? 'status-icon--success' : 'status-icon--danger'">
            {{ backendStatus ? 'check_circle' : 'cancel' }}
          </span>
          <span class="text-sm text-muted">Backend {{ backendStatus ? 'OK' : 'Error' }}</span>
        </div>
        <div class="status">
          <span class="material-symbols-outlined status-icon" :class="wsStatus ? 'status-icon--success' : 'status-icon--danger'">
            {{ wsStatus ? 'check_circle' : 'cancel' }}
          </span>
          <span class="text-sm text-muted">WS {{ wsStatus ? 'OK' : 'Error' }}</span>
        </div>
      </div>
      
      <div class="action-buttons">
        <AppTooltip v-if="botStore.status === 'stopped'" text="Conecta los WebSockets de Binance, Kraken y Bitfinex y activa la detección de arbitraje en tiempo real">
          <button class="btn-primary-light btn-toggle" :disabled="isToggling" @click="toggleBot(true)">
            <span>Iniciar Bot</span>
          </button>
        </AppTooltip>
        <button v-else-if="botStore.status === 'starting'" class="btn-dark btn-toggle" disabled>
          <div class="btn-spinner"></div>
        </button>
        <button v-else class="btn-primary-light btn-toggle" :disabled="isToggling" @click="toggleBot(false)">
          <div v-if="isToggling" class="btn-spinner" style="border-top-color: #000;"></div>
          <span v-else>Pausar Bot</span>
        </button>
      </div>

      <div class="profile-section">
        <span class="material-symbols-outlined avatar-icon">account_circle</span>
      </div>
    </div>
  </header>
  
  <AppSnackbar
    v-model="showSnackbar"
    :text="snackbarText"
    :action-text="snackbarActionText"
    :icon="snackbarIcon"
    :icon-color="snackbarIconColor"
    :loading="isToggling"
    @action="toggleBot(true)"
    @close="closeSnackbar"
  />
</template>

<style scoped>
.header {
  height: 80px;
  min-height: 80px;
  flex-shrink: 0;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 32px;
}

.header-title h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
  letter-spacing: -0.01em;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 32px;
}

.status-indicators {
  display: flex;
  gap: 24px;
}

.status {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 500;
}

.status-icon {
  font-size: 16px;
}

.status-icon--success { color: var(--color-success); }
.status-icon--danger { color: var(--color-danger); }

.action-buttons {
  display: flex;
  align-items: center;
  gap: 16px;
}

.btn-dark {
  background: #252836;
  color: var(--color-text-primary);
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-weight: 500;
  font-size: 13px;
  cursor: pointer;
  transition: background 150ms;
}
.btn-dark:hover:not(:disabled) { background: #303446; }
.btn-dark:disabled { opacity: 0.7; cursor: not-allowed; }

.btn-primary-light {
  background: var(--color-primary-light);
  color: #000000;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  font-weight: 600;
  font-size: 13px;
  cursor: pointer;
  transition: background 150ms;
}
.btn-primary-light:hover:not(:disabled) { 
  opacity: 0.85;
}
.btn-primary-light:disabled { opacity: 0.7; cursor: not-allowed; }

.btn-toggle {
  min-width: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.btn-text {
  background: transparent;
  color: var(--color-text-secondary);
  border: none;
  font-weight: 500;
  font-size: 13px;
  cursor: pointer;
}
.btn-text:hover { color: var(--color-text-primary); }

.profile-section {
  display: flex;
  align-items: center;
  gap: 16px;
  padding-left: 24px;
  border-left: 1px solid var(--color-border);
}

.avatar-icon {
  font-size: 32px;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: color 150ms;
}

.avatar-icon:hover {
  color: var(--color-text-primary);
}

.text-sm { font-size: 13px; }
</style>
