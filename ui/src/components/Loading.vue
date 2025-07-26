<script setup lang="ts">
import { cn } from '@tb-dev/vue-components';
import { LoaderCircleIcon } from 'lucide-vue-next';
import type { Component, HTMLAttributes } from 'vue';

const props = defineProps<{
  class?: HTMLAttributes['class'];
  icon?: Component;
  iconClass?: HTMLAttributes['class'];
}>();
</script>

<template>
  <div :class="cn('flex size-full items-center justify-center', props.class)">
    <component :is="icon ?? LoaderCircleIcon" :class="cn('loading-icon size-6', iconClass)" />
  </div>
</template>

<style scoped>
.loading-icon {
  animation-duration: 1500ms;
  animation-timing-function: linear;
  animation-iteration-count: infinite;
  animation-name: spin;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
