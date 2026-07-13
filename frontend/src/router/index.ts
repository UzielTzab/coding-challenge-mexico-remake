import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';
import DashboardView from '../views/DashboardView.vue';

const routes: Array<RouteRecordRaw> = [
  { path: '/', name: 'Dashboard', component: DashboardView, meta: { title: 'Dashboard' } },
  // { path: '/markets', name: 'Markets', component: () => import('../views/MarketsView.vue'), meta: { title: 'Mercados' } },
  { path: '/opportunities', name: 'Opportunities', component: () => import('../views/OpportunitiesView.vue'), meta: { title: 'Oportunidades' } },
  { path: '/operations', name: 'Operations', component: () => import('../views/OperationsView.vue'), meta: { title: 'Operaciones' } },
  { path: '/wallets', name: 'Wallets', component: () => import('../views/WalletsView.vue'), meta: { title: 'Wallets' } },
  { path: '/performance', name: 'Performance', component: () => import('../views/PerformanceView.vue'), meta: { title: 'Rendimiento' } },
  // { path: '/logs', name: 'Logs', component: () => import('../views/LogsView.vue'), meta: { title: 'Logs' } },
  { path: '/settings', name: 'Settings', component: () => import('../views/SettingsView.vue'), meta: { title: 'Configuración' } },
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
