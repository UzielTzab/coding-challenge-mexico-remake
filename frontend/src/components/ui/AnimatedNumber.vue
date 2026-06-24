<script setup lang="ts">
import { ref, watch, computed } from 'vue';

const props = defineProps<{
  value: number | string;
  format?: (val: any) => string;
}>();

const direction = ref<'up' | 'down'>('up');
const displayValue = ref(props.value);
const flashClass = ref('');
let flashTimeout: ReturnType<typeof setTimeout>;

const formattedChars = computed(() => {
  const str = props.format ? props.format(displayValue.value) : String(displayValue.value);
  return str.split('');
});

watch(() => props.value, (newVal, oldVal) => {
  const n1 = typeof newVal === 'string' ? parseFloat(newVal) : newVal;
  const n2 = typeof oldVal === 'string' ? parseFloat(oldVal) : oldVal;
  
  if (n1 > n2) {
    direction.value = 'up';
    flashClass.value = 'flash-up';
  } else if (n1 < n2) {
    direction.value = 'down';
    flashClass.value = 'flash-down';
  }
  displayValue.value = newVal;

  clearTimeout(flashTimeout);
  flashTimeout = setTimeout(() => {
    flashClass.value = '';
  }, 300);
});
</script>

<template>
  <div class="animated-number" :class="flashClass">
    <span v-for="(char, index) in formattedChars" :key="index" class="digit-wrapper">
      <transition :name="`slide-${direction}`" mode="out-in">
        <span :key="char" class="digit">{{ char }}</span>
      </transition>
    </span>
  </div>
</template>

<style scoped>
.animated-number {
  display: inline-flex;
  align-items: center;
  position: relative;
  vertical-align: bottom;
}

.digit-wrapper {
  display: inline-block;
  overflow: hidden;
  position: relative;
  /* Fixed width is tricky for variable width fonts, but inline-block handles it usually.
     We just let the container size to the digit. */
}

.digit {
  display: inline-block;
}

/* Slide UP: numbers go UP (value increases) */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.2s ease;
}
.slide-up-enter-from {
  transform: translateY(100%);
  opacity: 0;
}
.slide-up-leave-to {
  transform: translateY(-100%);
  opacity: 0;
}

/* Slide DOWN: numbers go DOWN (value decreases) */
.slide-down-enter-active,
.slide-down-leave-active {
  transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.2s ease;
}
.slide-down-enter-from {
  transform: translateY(-100%);
  opacity: 0;
}
.slide-down-leave-to {
  transform: translateY(100%);
  opacity: 0;
}

/* Flashes */
.flash-up {
  animation: flash-green 0.3s ease-out;
}
.flash-down {
  animation: flash-red 0.3s ease-out;
}

@keyframes flash-green {
  0% { color: var(--color-success); text-shadow: 0 0 8px rgba(16, 185, 129, 0.5); }
  100% { color: inherit; text-shadow: none; }
}

@keyframes flash-red {
  0% { color: var(--color-danger); text-shadow: 0 0 8px rgba(239, 68, 68, 0.5); }
  100% { color: inherit; text-shadow: none; }
}
</style>
