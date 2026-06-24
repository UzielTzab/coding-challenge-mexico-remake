import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Opportunity } from '../types/domain.types';

export const useOpportunitiesStore = defineStore('opportunities', () => {
  const items = ref<Opportunity[]>([]);
  const totalPnl = ref(0);
  const pnlHistory = ref<{date: string, value: number, time: number}[]>([]);
  
  const summary = ref({
    global_win_rate: 0,
    trades_count: 0,
    discarded_opportunities: 0,
    opportunities_count: 0,
    average_cost: 0
  });

  const prepend = (opp: Opportunity) => {
    items.value.unshift(opp);
    if (items.value.length > 50) {
      items.value.pop();
    }
  };

  const updateGlobalPnl = (newPnl: number) => {
    totalPnl.value = newPnl;
    const now = new Date();
    pnlHistory.value.push({
      date: now.toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute:'2-digit', second:'2-digit' }),
      value: totalPnl.value,
      time: Math.floor(now.getTime() / 1000)
    });
    if (pnlHistory.value.length > 20) {
      pnlHistory.value.shift();
    }
  };

  const setInitialPnl = (initialPnl: number) => {
    totalPnl.value = initialPnl;
    if (pnlHistory.value.length === 0) {
      const now = new Date();
      pnlHistory.value.push({
        date: now.toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute:'2-digit', second:'2-digit' }),
        value: initialPnl,
        time: Math.floor(now.getTime() / 1000)
      });
    }
  };

  const setSummary = (s: any) => {
    summary.value = { ...summary.value, ...s };
  };

  return { items, prepend, totalPnl, pnlHistory, setInitialPnl, summary, setSummary, updateGlobalPnl };
});
