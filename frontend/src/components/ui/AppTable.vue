<script setup lang="ts">
import AppEmptyState from './AppEmptyState.vue';
import AppSkeleton from './AppSkeleton.vue';

interface Props {
  columns: Array<{ key: string; label: string; align?: 'left' | 'center' | 'right' }>;
  data: any[];
  loading?: boolean;
}

defineProps<Props>();
</script>

<template>
  <div class="app-table-container">
    <table v-if="data.length > 0 || loading" class="app-table">
      <thead>
        <tr>
          <th 
            v-for="col in columns" 
            :key="col.key"
            :style="{ textAlign: col.align || 'left' }"
          >
            {{ col.label }}
          </th>
        </tr>
      </thead>
      <tbody v-if="!loading">
        <tr v-for="(row, index) in data" :key="index">
          <td 
            v-for="col in columns" 
            :key="col.key"
            :style="{ textAlign: col.align || 'left' }"
          >
            <slot :name="'cell-' + col.key" :item="row">
              {{ row[col.key] }}
            </slot>
          </td>
        </tr>
      </tbody>
      <tbody v-else>
        <tr v-for="i in 5" :key="i">
          <td v-for="col in columns" :key="col.key">
            <AppSkeleton height="16px" width="70%" borderRadius="4px" />
          </td>
        </tr>
      </tbody>
    </table>
    
    <AppEmptyState v-if="!loading && data.length === 0" message="No hay datos disponibles" />
  </div>
</template>

<style scoped>
.app-table-container {
  width: 100%;
  overflow-x: auto;
}

.app-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--text-sm, 13px);
}

.app-table th {
  color: var(--color-text-muted);
  font-weight: 600;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
  white-space: nowrap;
}

.app-table td {
  padding: 14px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.02);
  color: var(--color-text-primary);
  font-family: var(--font-mono, monospace);
}

.app-table tbody tr:hover td {
  background: rgba(255, 255, 255, 0.01);
}

.table-loading {
  padding: 32px;
  text-align: center;
}
</style>
