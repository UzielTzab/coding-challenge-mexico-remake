import type { ArbitrageOpportunity } from '../types/domain.types';
import api from './http';

export const getOpportunities = async (params?: any) => {
  const response = await api.get('/api/opportunities/', { params });
  return response.data;
};
