import { ref } from 'vue';

export const useApi = <T>() => {
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  const execute = async (apiCall: () => Promise<T>) => {
    loading.value = true;
    error.value = null;
    try {
      const data = await apiCall();
      return data;
    } catch (err: any) {
      error.value = err.message || 'Error en la llamada a la API';
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return { loading, error, execute };
};
