import type { MarketData, Opportunity, Trade, Wallet } from './domain.types';

export interface WsMessage<T = any> {
  type: string;
  payload: T;
  timestamp?: string;
}

export type WsMarketUpdate = WsMessage<MarketData>;
export type WsOpportunityDetected = WsMessage<Opportunity>;
export type WsTradeExecuted = WsMessage<Trade>;
export type WsWalletUpdated = WsMessage<Wallet>;
export type WsSystemLog = WsMessage<{ level: string; message: string }>;
