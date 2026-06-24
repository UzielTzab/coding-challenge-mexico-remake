<script setup lang="ts">
defineProps<{
  width?: string;
  height?: string;
  borderRadius?: string;
}>();
</script>

<template>
  <div 
    class="skeleton" 
    :style="{ 
      width: width || '100%', 
      height: height || '20px', 
      borderRadius: borderRadius || '4px' 
    }"
  ></div>
</template>

<style scoped>
.skeleton {
  background: var(--color-bg-card-soft);
  position: relative;
  overflow: hidden;
}

.skeleton::after {
  content: '';
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  transform: translateX(-100%);
  background-image: linear-gradient(
    90deg,
    rgba(255, 255, 255, 0) 0,
    rgba(255, 255, 255, 0.05) 20%,
    rgba(255, 255, 255, 0.08) 60%,
    rgba(255, 255, 255, 0)
  );
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  100% {
    transform: translateX(100%);
  }
}
</style>
