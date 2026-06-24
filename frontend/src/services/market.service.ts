import type { MarketSnapshot } from '../types/domain.types';
import get from './http';

export const getMarkets = async () => {
  const { data } = await get<MarketSnapshot[]>('/markets');
  return data;
};
