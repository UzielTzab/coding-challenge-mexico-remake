<script setup lang="ts">
import RiskSliders from '../components/Settings/RiskSliders.vue'
</script>

<template>
  <div class="settings-view">
    <header>
      <h2>System Configuration</h2>
      <p class="subtitle">Adjust risk parameters and algorithm thresholds.</p>
    </header>

    <div class="settings-container">
      <div class="panel glass">
        <h3>Risk Management</h3>
        <RiskSliders />
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

header {
  margin-bottom: 8px;
}

h2 {
  margin: 0 0 8px 0;
  font-size: 1.5rem;
  font-weight: 600;
}

.subtitle {
  margin: 0;
  color: var(--color-text-secondary);
  font-size: 0.9rem;
}

.settings-container {
  display: grid;
  grid-template-columns: 1fr;
  max-width: 800px;
}

.panel {
  background-color: var(--color-bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
  padding: 24px;
}

.glass {
  background: rgba(21, 24, 38, 0.7);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

h3 {
  margin: 0 0 24px 0;
  font-size: 1.1rem;
  color: var(--color-text-primary);
  font-weight: 500;
  border-bottom: 1px solid var(--color-border);
  padding-bottom: 12px;
}
</style>
