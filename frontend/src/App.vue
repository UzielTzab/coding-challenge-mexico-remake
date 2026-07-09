<script setup lang="ts">
import { onMounted } from 'vue';
import AppShell from './components/layout/AppShell.vue';
import GlobalSnackbars from './components/ui/GlobalSnackbars.vue';
import { useDashboardSocket } from './composables/useDashboardSocket';

const { connect } = useDashboardSocket();

onMounted(() => {
  connect();
});
</script>

<template>
  <AppShell />
  <GlobalSnackbars />
</template>

<style>
/* Los estilos globales están en src/styles/globals.css */
</style>
