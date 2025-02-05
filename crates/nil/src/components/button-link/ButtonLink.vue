<script setup lang="ts">
import type { VNode } from 'vue';
import { cn } from '@/lib/utils';
import { RouterLink } from 'vue-router';
import type { Route } from '@/router/types';
import { Button, type ButtonSize, type ButtonVariant } from '../button';

interface Props {
  buttonClass?: string;
  label?: string;
  size?: ButtonSize;
  to: Route;
  variant?: ButtonVariant;
}

withDefaults(defineProps<Props>(), {
  size: 'default',
  variant: 'ghost',
});

defineSlots<{ default?: () => VNode }>();
</script>

<template>
  <RouterLink :to="{ name: to }">
    <Button :variant :size :class="cn('size-full', buttonClass)">
      <span v-if="label">{{ label }}</span>
      <slot v-else></slot>
    </Button>
  </RouterLink>
</template>
