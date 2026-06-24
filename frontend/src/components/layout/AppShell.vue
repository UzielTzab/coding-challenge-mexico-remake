<script setup lang="ts">
import AppSidebar from './AppSidebar.vue';
import AppHeader from './AppHeader.vue';
</script>

<template>
  <div class="app-shell">
    <AppSidebar />
    <main class="app-main">
      <AppHeader />
      <div class="app-content">
        <router-view />
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-shell {
  height: 100vh;
  display: grid;
  grid-template-columns: 200px 1fr;
  background: var(--color-bg-base);
}

.app-main {
  min-width: 0;
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.app-content {
  padding: var(--content-padding, 24px);
  flex-grow: 1;
  overflow-y: auto;
}
</style>
