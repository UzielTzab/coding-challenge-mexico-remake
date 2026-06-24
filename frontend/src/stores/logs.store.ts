import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface SystemLog {
  id: string;
  level: string;
  message: string;
  timestamp: string;
}

export const useLogsStore = defineStore('logs', () => {
  const items = ref<SystemLog[]>([]);
  
  const prepend = (log: SystemLog) => {
    items.value.unshift(log);
  };

  return { items, prepend };
});
