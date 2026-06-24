import { watch } from 'vue';
import { useWebSocket } from './useWebSocket';
import { useMarketStore } from '../stores/market.store';
import { useLogsStore } from '../stores/logs.store';
import { useOpportunitiesStore } from '../stores/opportunities.store';
import { useUiStore } from '../stores/ui.store';

export const useDashboardSocket = () => {
  const wsUrl = (import.meta.env.VITE_WS_BASE_URL || 'ws://localhost:8000') + '/ws/dashboard/';
  const { ws, isConnected, connect } = useWebSocket(wsUrl);
  
  const marketStore = useMarketStore();
  const logsStore = useLogsStore();
  const oppStore = useOpportunitiesStore();
  const uiStore = useUiStore();
  
  // Actually, I'll just map the events according to what is present for now.
  watch(ws, (socket) => {
    if (socket) {
      socket.onmessage = (event: MessageEvent) => {
        const data = JSON.parse(event.data);
        
        switch (data.type) {
          case 'market_update':
            // console.log(`[${new Date().toISOString()}] WS DATA RECEIVED for ${data.exchange}. Bid: ${data.best_bid}, Ask: ${data.best_ask}`);
            marketStore.upsertSnapshot({
              exchange: data.exchange,
              pair: data.symbol,
              bid: parseFloat(data.best_bid),
              ask: parseFloat(data.best_ask),
              bidVolume: parseFloat(data.bid_volume),
              askVolume: parseFloat(data.ask_volume),
              timestamp: new Date().toISOString(),
              latency_ms: data.latency_ms
            });
            break;
          case 'system_log_created':
            logsStore.prepend({
              id: Math.random().toString(36).substring(2, 9),
              level: data.level,
              message: data.message,
              timestamp: data.created_at
            });
            break;
          case 'opportunity_detected':
            const opp = data.opportunity || data;
            oppStore.prepend(opp);
            
            if (opp.status === 'executed' || opp.status === 'profitable') {
              uiStore.showSnackbar(`Arbitraje Ejecutado: <strong>+$${parseFloat(opp.net_profit).toFixed(2)}</strong>`, 'success');
            }
            break;
          case 'trade_simulated':
            // tradesStore.prepend(data);
            break;
          case 'wallet_updated':
            // walletsStore.update(data);
            break;
          case 'bot_status_changed':
            oppStore.setSummary({
              global_win_rate: parseFloat(data.win_rate_percent) || 0,
              trades_count: data.total_trades || 0,
              discarded_opportunities: data.discarded_opportunities || 0,
              opportunities_count: (data.total_trades || 0) + (data.discarded_opportunities || 0),
              average_cost: parseFloat(data.total_fees_usd) || 0
            });
            oppStore.updateGlobalPnl(parseFloat(data.total_pnl_usd) || 0);
            break;
        }
      };
    }
  });

  return { isConnected, connect };
};
