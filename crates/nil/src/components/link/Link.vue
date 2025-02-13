<script setup lang="ts">
import type { VNode } from 'vue';
import { RouterLink } from 'vue-router';
import type { Route } from '@/router/types';

type Props = {
  label?: string;
  to: Route;
};

defineProps<Props>();

defineSlots<{ default?: () => VNode }>();
</script>

<template>
  <RouterLink :to="{ name: to }">
    <span v-if="label">{{ label }}</span>
    <slot v-else></slot>
  </RouterLink>
</template>
