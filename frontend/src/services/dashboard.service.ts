import type { PerformanceSnapshot, Exchange } from '../types/domain.types';
import api from './http';

export const getDashboardSummary = async () => {
  const { data } = await api.get<PerformanceSnapshot>('/api/analytics/performance/');
  return data;
};

export const getExchanges = async () => {
  const { data } = await api.get<Exchange[]>('/api/exchanges/');
  return data;
};
