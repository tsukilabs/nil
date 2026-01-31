<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { HTMLAttributes, VNode } from 'vue';
import { LoaderCircleIcon } from 'lucide-vue-next';
import { Button, type ButtonSize, type ButtonVariant, type ClassValue, cn } from '@tb-dev/vue-components';

defineProps<{
  variant?: ButtonVariant;
  size?: Exclude<ButtonSize, 'icon' | 'icon-sm' | 'icon-lg'>;
  disabled?: boolean;
  loading: boolean;
  role?: HTMLAttributes['role'];
  tabindex?: HTMLAttributes['tabindex'];
  iconClass?: ClassValue;
  onClick?: () => MaybePromise<void>;
}>();

defineSlots<{
  default?: () => VNode;
}>();
</script>

<template>
  <Button :variant :size :disabled :role :tabindex @click="onClick">
    <slot></slot>
    <LoaderCircleIcon v-if="loading" :class="cn('loading-icon size-4', iconClass)" />
  </Button>
</template>

<style scoped>
.loading-icon {
  animation-duration: 1500ms;
  animation-timing-function: linear;
  animation-iteration-count: infinite;
  animation-name: loading-icon-spin;
}

@keyframes loading-icon-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
