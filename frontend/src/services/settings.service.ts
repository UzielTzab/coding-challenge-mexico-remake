import api from './http';

export const getSettings = async () => {
  const { data } = await api.get<Record<string, any>>('/api/settings/');
  return data;
};

export const updateSettings = async (id: string, payload: any) => {
  const { data } = await api.patch<Record<string, any>>(`/api/settings/${id}/`, payload);
  return data;
};
