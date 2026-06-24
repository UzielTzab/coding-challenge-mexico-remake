<script setup lang="ts">
import { ref } from 'vue'

const slippage = ref(0.1)
const rebalanceThreshold = ref(1.5)
const maxPosition = ref(50000)

const saveSettings = () => {
  // TODO: Implement API call to save settings
  console.log('Settings saved:', {
    slippage: slippage.value,
    rebalanceThreshold: rebalanceThreshold.value,
    maxPosition: maxPosition.value
  })
}
</script>

<template>
  <div class="risk-sliders">
    <div class="control-group">
      <div class="label-row">
        <label>Max Slippage (%)</label>
        <span class="value">{{ slippage }}%</span>
      </div>
      <input type="range" v-model="slippage" min="0.01" max="1" step="0.01" class="slider" />
    </div>

    <div class="control-group">
      <div class="label-row">
        <label>Rebalance Threshold (%)</label>
        <span class="value">{{ rebalanceThreshold }}%</span>
      </div>
      <input type="range" v-model="rebalanceThreshold" min="0.5" max="5" step="0.1" class="slider" />
    </div>

    <div class="control-group">
      <div class="label-row">
        <label>Max Position Size (USD)</label>
        <span class="value">${{ maxPosition.toLocaleString() }}</span>
      </div>
      <input type="range" v-model="maxPosition" min="1000" max="100000" step="1000" class="slider" />
    </div>

    <button @click="saveSettings" class="btn-primary">Apply Configuration</button>
  </div>
</template>

<style scoped>
.risk-sliders {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.control-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.label-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

label {
  font-weight: 500;
  color: var(--color-text-secondary);
}

.value {
  font-family: var(--font-mono);
  font-weight: 600;
  color: var(--color-text-primary);
  background: var(--color-bg-terminal);
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
}

/* Custom Slider */
.slider {
  -webkit-appearance: none;
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--color-bg-terminal);
  outline: none;
  border: 1px solid var(--color-border);
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
  transition: all 0.2s;
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
  background: var(--color-primary-light);
}

.btn-primary {
  margin-top: 16px;
  padding: 12px 24px;
  background: var(--gradient-primary);
  color: #fff;
  border: none;
  border-radius: var(--radius-md);
  font-weight: 600;
  font-size: 1rem;
  cursor: pointer;
  transition: opacity 0.2s;
  align-self: flex-start;
}

.btn-primary:hover {
  opacity: 0.9;
}
</style>
