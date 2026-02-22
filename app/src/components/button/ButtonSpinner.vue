<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { HTMLAttributes, VNode } from 'vue';
import { Button, type ButtonSize, type ButtonVariant, type ClassValue, Spinner } from '@tb-dev/vue-components';

defineProps<{
  variant?: ButtonVariant;
  size?: Exclude<ButtonSize, `icon${string}`>;
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
    <Spinner v-if="loading" :class="iconClass" />
    <slot></slot>
  </Button>
</template>
