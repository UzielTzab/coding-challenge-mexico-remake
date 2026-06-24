import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useBotStore = defineStore('bot', () => {
  const status = ref<'running' | 'starting' | 'stopped'>('stopped');
  const latency = ref(0);
  
  const setStatus = (newStatus: 'running' | 'starting' | 'stopped') => {
    status.value = newStatus;
  };

  const setLatency = (ms: number) => {
    latency.value = ms;
  };

  return { status, latency, setStatus, setLatency };
});
