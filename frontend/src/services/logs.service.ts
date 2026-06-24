import type { SystemLog } from '../types/domain.types';
import api from './http';

export const getLogs = async () => {
  const { data } = await api.get<SystemLog[]>('/api/logs/');
  return data;
};
