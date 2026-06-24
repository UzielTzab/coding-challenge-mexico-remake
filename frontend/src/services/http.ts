import axios from 'axios';

const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000',
  timeout: 10000,
});

export const get = async <T = any>(url: string)=>{
  const {data} = await api.get<T>(url);
  return data as T;
} 

export default api;