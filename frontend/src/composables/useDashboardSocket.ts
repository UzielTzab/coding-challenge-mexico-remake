import { watch } from 'vue';
import { useWebSocket } from './useWebSocket';
import { useMarketStore } from '../stores/market.store';
import { useLogsStore } from '../stores/logs.store';
import { useOpportunitiesStore } from '../stores/opportunities.store';
import { useUiStore } from '../stores/ui.store';
import { useBotStore } from '../stores/bot.store';

export const useDashboardSocket = () => {
  const wsUrl = (import.meta.env.VITE_WS_BASE_URL || 'ws://localhost:8000') + '/ws/market';
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
              exchange: data.exchange.toLowerCase(),
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
          case 'REBALANCE':
            uiStore.showSnackbar(`Drenaje de Liquidez en <strong>${data.target}</strong>. Iniciando Enrutamiento Triangular (${data.method}) para ${data.amount} ${data.asset}.`, 'critical', 6000);
            break;
          case 'EMERGENCY_HEDGE':
            uiStore.showSnackbar(`<strong>Emergency Hedge:</strong> Slippage crítico detectado. Cubriendo posición para evitar pérdida mayor.`, 'critical', 6000);
            break;
          case 'DELTA_HEDGE':
            uiStore.showSnackbar(`<strong>Delta Neutrality:</strong> ${data.message} (${data.net_exposure} BTC)`, 'critical', 6000);
            break;
          case 'opportunity_detected': {
            const opp = data.opportunity || data;
            const botStore = useBotStore();
            
            // Si el bot está apagado, la oportunidad simplemente se registra como descartada/ignorada
            if (botStore.status !== 'running') {
              opp.status = 'discarded';
              oppStore.prepend(opp);
              oppStore.setSummary({
                discarded_opportunities: oppStore.summary.discarded_opportunities + 1,
                opportunities_count: oppStore.summary.opportunities_count + 1
              });
              break; // No mostramos snackbar porque el bot está apagado
            }

            oppStore.prepend(opp);
            
            if (opp.is_partial_fill || opp.status === 'emergency_hedge') {
              uiStore.showSnackbar(`<strong>Emergency Hedge:</strong> Slippage crítico en arbitraje. Cubriendo posición.`, 'critical', 6000);
            } else if (opp.status === 'legging_hedge') {
              uiStore.showSnackbar(`<strong>Legging Risk Detectado:</strong> Pata 2 falló. Ejecutando Market Dump.`, 'warning', 6000);
            }
            
            // UPDATE PNL & STATS
            if (opp.status === 'executed' || opp.status === 'legging_hedge' || opp.status === 'emergency_hedge') {
              oppStore.updateGlobalPnl(oppStore.totalPnl + parseFloat(opp.net_profit || 0));
              oppStore.setSummary({
                trades_count: oppStore.summary.trades_count + 1,
                opportunities_count: oppStore.summary.opportunities_count + 1
              });
            } else if (opp.status === 'discarded') {
              oppStore.setSummary({
                discarded_opportunities: oppStore.summary.discarded_opportunities + 1,
                opportunities_count: oppStore.summary.opportunities_count + 1
              });
            }
            break;
          }
          case 'trade_simulated':
            if (data.is_partial_fill) {
               uiStore.showSnackbar(`<strong>Emergency Hedge:</strong> Slippage detectado en simulación.`, 'critical', 6000);
            }
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
