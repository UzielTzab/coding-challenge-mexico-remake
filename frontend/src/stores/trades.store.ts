import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Trade } from '../types/domain.types';

export const useTradesStore = defineStore('trades', () => {
  const items = ref<Trade[]>([]);
  
  const prepend = (trade: Trade) => {
    items.value.unshift(trade);
  };

  return { items, prepend };
});
