<script setup lang="ts">
defineProps<{
  text: string;
}>();
</script>

<template>
  <div class="tooltip-wrapper">
    <slot></slot>
    <div class="custom-tooltip">
      {{ text }}
    </div>
  </div>
</template>

<style scoped>
.tooltip-wrapper {
  position: relative;
  display: inline-block;
}

.custom-tooltip {
  visibility: hidden;
  width: 250px;
  background-color: var(--color-bg-card);
  color: var(--color-text-secondary);
  text-align: center;
  border-radius: 8px;
  padding: 12px;
  position: absolute;
  z-index: 1000;
  top: 125%;
  left: 50%;
  margin-left: -125px;
  opacity: 0;
  transition: opacity 0.3s;
  font-size: 13px;
  line-height: 1.5;
  border: 1px solid var(--color-border);
  box-shadow: 0 4px 12px rgba(0,0,0,0.3);
}

.custom-tooltip::after {
  content: "";
  position: absolute;
  bottom: 100%;
  left: 50%;
  margin-left: -6px;
  border-width: 6px;
  border-style: solid;
  border-color: transparent transparent var(--color-border) transparent;
}

.tooltip-wrapper:hover .custom-tooltip {
  visibility: visible;
  opacity: 1;
}
</style>
