import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Wallet } from '../types/domain.types';

export const useWalletsStore = defineStore('wallets', () => {
  const items = ref<Record<string, Wallet>>({});
  
  const update = (wallet: Wallet) => {
    items.value[`${wallet.exchange}-${wallet.asset}`] = wallet;
  };

  return { items, update };
});
