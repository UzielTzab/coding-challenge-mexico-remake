import { ref, onUnmounted } from 'vue';

export const useWebSocket = (url: string) => {
  const ws = ref<WebSocket | null>(null);
  const isConnected = ref<boolean>(false);

  const connect = () => {
    ws.value = new WebSocket(url);
    ws.value.onopen = () => isConnected.value = true;  
    ws.value.onclose = () => isConnected.value = false;
  };

  const close = () => {
    if (ws.value) {
      ws.value.close();
    }
  };

  onUnmounted(() => {
    close();
  });

  return { ws, isConnected, connect, close };
};
