import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { MarketData } from '../types/domain.types';

export const useMarketStore = defineStore('market', () => {
  const snapshots = ref<Record<string, MarketData>>({});
  
  const upsertSnapshot = (data: MarketData) => {
    const key = `${data.exchange}-${data.pair}`;
    if (snapshots.value[key]) {
      snapshots.value[key] = {
        ...snapshots.value[key],
        ...data
      };
    } else {
      snapshots.value[key] = data;
    }
  };

  return { snapshots, upsertSnapshot };
});
