import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const slippage = ref(0.1)
  const rebalanceThreshold = ref(1.5)
  const maxPosition = ref(50000)

  const fetchConfiguration = async () => {
    try {
      const response = await fetch('http://localhost:8000/api/settings')
      if (response.ok) {
        const data = await response.json()
        if (data.slippage !== undefined) slippage.value = data.slippage
        if (data.rebalanceThreshold !== undefined) rebalanceThreshold.value = data.rebalanceThreshold
        if (data.maxPosition !== undefined) maxPosition.value = data.maxPosition
      }
    } catch (e) {
      console.error('Failed to fetch settings:', e)
    }
  }

  const saveConfiguration = async () => {
    try {
      await fetch('http://localhost:8000/api/settings', {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          slippage: slippage.value,
          rebalanceThreshold: rebalanceThreshold.value,
          maxPosition: maxPosition.value
        })
      })
    } catch (e) {
      console.error('Failed to save settings:', e)
    }
  }

  return {
    slippage,
    rebalanceThreshold,
    maxPosition,
    fetchConfiguration,
    saveConfiguration
  }
})
