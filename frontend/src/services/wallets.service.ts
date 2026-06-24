import type { Wallet, WalletMovement } from '../types/domain.types';
import get from './http';
import api from './http';

export const getWallets = async () => {
  const { data } = await get<Wallet[]>('/api/wallets/');
  return data;
};

export const getWalletMovements = async (params?: any) => {
  const { data } = await api.get<WalletMovement[]>('/api/wallets/movements/', { params });
  return data;
};
