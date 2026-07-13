<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useBotStore } from '../../stores/bot.store';

const route = useRoute();
const botStore = useBotStore();

const links = [
  { name: 'Dashboard', path: '/', icon: 'dashboard' },
  { name: 'Oportunidades', path: '/opportunities', icon: 'monitoring' },
  { name: 'Operaciones', path: '/operations', icon: 'swap_horiz' },
  { name: 'Wallets', path: '/wallets', icon: 'account_balance_wallet' },
  { name: 'Rendimiento', path: '/performance', icon: 'leaderboard' },
  { name: 'Configuración', path: '/settings', icon: 'settings' },
];
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-logo">
      <h2>ArbiBTC</h2>
      <span class="text-muted text-sm">Arbitrage Bot</span>
    </div>
    
    <nav class="sidebar-nav">
      <router-link 
        v-for="link in links" 
        :key="link.path"
        :to="link.path"
        class="nav-link"
        :class="{ active: route.path === link.path }"
      >
        <span class="material-symbols-outlined nav-icon">{{ link.icon }}</span>
        {{ link.name }}
      </router-link>
    </nav>
    
    <div class="sidebar-footer">
      <div class="footer-status">
        <span class="material-symbols-outlined status-icon">play_circle</span>
        <span class="text-sm">Modo: Simulación</span>
      </div>
      <div class="footer-status">
        <span class="status-dot" :style="{ background: botStore.status === 'running' ? 'var(--color-success)' : (botStore.status === 'starting' ? 'var(--color-warning)' : 'var(--color-danger)') }"></span>
        <span class="text-sm">Bot: {{ botStore.status === 'running' ? 'En Ejecución' : (botStore.status === 'starting' ? 'Iniciando...' : 'Detenido') }}</span>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 100%;
  background: var(--color-bg-tertiary);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.sidebar-logo {
  height: 80px;
  padding: 0 24px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.sidebar-logo h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
  letter-spacing: -0.02em;
}

.sidebar-nav {
  padding: 16px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex-grow: 1;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  color: var(--color-text-secondary);
  text-decoration: none;
  border-radius: var(--radius-md);
  font-size: 14px;
  font-weight: 500;
  transition: all 160ms ease;
}

.nav-icon {
  font-size: 20px;
  opacity: 0.8;
}

.nav-link:hover {
  background: transparent;
  color: var(--color-text-primary);
}

.nav-link.active {
  background: var(--color-bg-secondary);
  color: var(--color-primary-light);
  font-weight: 600;
}

.nav-link.active .nav-icon {
  opacity: 1;
}

.sidebar-footer {
  padding: 24px;
  border-top: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.footer-status {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--color-text-secondary);
}

.status-icon {
  font-size: 18px;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-danger);
}

.footer-version {
  margin-top: 8px;
  font-size: 11px;
  color: var(--color-text-muted);
}

.text-sm { font-size: 12px; }
</style>
