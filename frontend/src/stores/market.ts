import { defineStore } from 'pinia'
import { ref, shallowRef } from 'vue'

export interface CandleData {
  time: number | string; // lightweight-charts accepts timestamp (seconds) or string ('YYYY-MM-DD')
  open: number;
  high: number;
  low: number;
  close: number;
}

type Subscriber = (candle: CandleData) => void;

export const useMarketStore = defineStore('market', () => {
  const currentPnL = ref(0)
  const volume24h = ref(0)
  
  // Reactively track connection status if needed
  const isConnected = ref(false)
  const wsError = ref<Error | null>(null)

  // Use a fast channel (subscribers array) instead of full reactive state for 60FPS updates
  // This avoids Vue's reactivity overhead on high-frequency data
  const subscribers = shallowRef<Subscriber[]>([])

  const subscribe = (callback: Subscriber) => {
    subscribers.value.push(callback)
    return () => {
      subscribers.value = subscribers.value.filter(cb => cb !== callback)
    }
  }

  // WebSocket initialization
  const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const ws = new WebSocket(`${wsProtocol}//${window.location.host}/ws/market`)

  ws.onopen = () => {
    isConnected.value = true
    wsError.value = null
  }

  ws.onclose = () => {
    isConnected.value = false
  }

  ws.onerror = (e) => {
    wsError.value = e as unknown as Error
  }

  let currentCandle: CandleData | null = null;
  let lastCandleTime = 0;

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data)
      
      // Update reactive UI stats if needed (throttle if it's too frequent)
      if (data.pnl !== undefined) {
        currentPnL.value = data.pnl
      }
      if (data.volume !== undefined) {
        volume24h.value = data.volume
      }

      // Aggregate tick data into OHLC candles
      if (data.type === 'market_update' && data.best_bid && data.best_ask) {
        const midPrice = (data.best_bid + data.best_ask) / 2;
        const nowSec = Math.floor(Date.now() / 1000);
        
        if (!currentCandle || nowSec > lastCandleTime) {
          currentCandle = {
            time: nowSec as number,
            open: currentCandle ? currentCandle.close : midPrice,
            high: currentCandle ? currentCandle.close : midPrice,
            low: currentCandle ? currentCandle.close : midPrice,
            close: midPrice
          };
          lastCandleTime = nowSec;
        } else {
          currentCandle.high = Math.max(currentCandle.high, midPrice);
          currentCandle.low = Math.min(currentCandle.low, midPrice);
          currentCandle.close = midPrice;
        }
        
        // Notify chart components directly without triggering Vue re-renders
        for (let i = 0; i < subscribers.value.length; i++) {
          subscribers.value[i]({...currentCandle})
        }
      } else if (data.time && data.open && data.high && data.low && data.close) {
        const candle: CandleData = {
          time: data.time,
          open: data.open,
          high: data.high,
          low: data.low,
          close: data.close
        }
        
        for (let i = 0; i < subscribers.value.length; i++) {
          subscribers.value[i](candle)
        }
      }
    } catch (e) {
      console.error('Failed to parse WebSocket message', e)
    }
  }

  return {
    currentPnL,
    volume24h,
    isConnected,
    subscribe
  }
})
