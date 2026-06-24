import type { SimulatedTrade } from '../types/domain.types';
import get from './http';

export const getTrades = async (params?: any) => {
  const { data } = await get<SimulatedTrade[]>('/api/trades', { params });
  return data;
};
