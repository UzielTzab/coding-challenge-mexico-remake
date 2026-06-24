<script setup lang="ts">
// Dummy data
const asks = [
  { price: 64200.50, amount: 1.2, total: 77040.60 },
  { price: 64195.00, amount: 0.5, total: 32097.50 },
  { price: 64190.20, amount: 2.1, total: 134799.42 },
]

const bids = [
  { price: 64180.00, amount: 0.8, total: 51344.00 },
  { price: 64175.50, amount: 1.5, total: 96263.25 },
  { price: 64170.10, amount: 3.2, total: 205344.32 },
]
</script>

<template>
  <div class="orderbook">
    <div class="header-row">
      <span>Price (USDT)</span>
      <span>Amount (BTC)</span>
      <span>Total</span>
    </div>
    
    <div class="book-side asks">
      <div v-for="(ask, idx) in asks" :key="'ask'+idx" class="row ask-row">
        <span class="price danger">{{ ask.price.toFixed(2) }}</span>
        <span>{{ ask.amount.toFixed(4) }}</span>
        <span>{{ ask.total.toFixed(2) }}</span>
      </div>
    </div>
    
    <div class="spread">
      <span>Spread</span>
      <span>10.20</span>
    </div>
    
    <div class="book-side bids">
      <div v-for="(bid, idx) in bids" :key="'bid'+idx" class="row bid-row">
        <span class="price success">{{ bid.price.toFixed(2) }}</span>
        <span>{{ bid.amount.toFixed(4) }}</span>
        <span>{{ bid.total.toFixed(2) }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.orderbook {
  display: flex;
  flex-direction: column;
  font-family: var(--font-mono);
  font-size: 0.8rem;
  height: 100%;
}

.header-row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  padding: 8px 0;
  color: var(--color-text-muted);
  border-bottom: 1px solid var(--color-border);
  margin-bottom: 8px;
}

.header-row span:last-child, .row span:last-child {
  text-align: right;
}

.header-row span:nth-child(2), .row span:nth-child(2) {
  text-align: right;
}

.book-side {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  padding: 4px 0;
  position: relative;
}

.price {
  font-weight: 500;
}

.danger {
  color: var(--color-danger);
}

.success {
  color: var(--color-success);
}

.spread {
  text-align: center;
  padding: 12px 0;
  margin: 8px 0;
  font-weight: 700;
  color: var(--color-text-primary);
  border-top: 1px dashed var(--color-border);
  border-bottom: 1px dashed var(--color-border);
  display: flex;
  justify-content: center;
  gap: 8px;
}
</style>
