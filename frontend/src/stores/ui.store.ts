import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface SnackbarMessage {
  id: number;
  text: string;
  type: 'success' | 'error' | 'info' | 'warning' | 'critical';
}

export const useUiStore = defineStore('ui', () => {
  const snackbars = ref<SnackbarMessage[]>([]);
  let nextId = 0;

  function showSnackbar(text: string, type: 'success' | 'error' | 'info' | 'warning' | 'critical' = 'info', duration = 4000) {
    const id = nextId++;
    snackbars.value.push({ id, text, type });
    setTimeout(() => {
      removeSnackbar(id);
    }, duration);
  }

  function removeSnackbar(id: number) {
    snackbars.value = snackbars.value.filter(s => s.id !== id);
  }

  return { snackbars, showSnackbar, removeSnackbar };
});
