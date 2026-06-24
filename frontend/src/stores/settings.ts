import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const slippage = ref(0.1)
  const rebalanceThreshold = ref(1.5)
  const maxPosition = ref(50000)

  // TODO: Implement API calls to Postgres/Redis via REST
  const saveConfiguration = async () => {
    // API logic
  }

  return {
    slippage,
    rebalanceThreshold,
    maxPosition,
    saveConfiguration
  }
})
