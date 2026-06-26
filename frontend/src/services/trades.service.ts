import type { SimulatedTrade } from '../types/domain.types';
import api from './http';

export const getTrades = async (params?: any) => {
  const response = await api.get('/api/trades', { params });
  return response.data;
};
