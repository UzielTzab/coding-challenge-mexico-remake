import type { ArbitrageOpportunity } from '../types/domain.types';
import get from './http';

export const getOpportunities = async (params?: any) => {
  const { data } = await get<ArbitrageOpportunity[]>('/api/opportunities/', { params });
  return data;
};
