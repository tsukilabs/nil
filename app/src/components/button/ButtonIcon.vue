<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { Component, HTMLAttributes, VNode } from 'vue';
import { Button, type ButtonSize, type ButtonVariant } from '@ui/button';

interface Props {
  disabled?: boolean;
  icon?: Component;
  role?: HTMLAttributes['role'];
  size?: ButtonSize;
  tabindex?: HTMLAttributes['tabindex'];
  variant?: ButtonVariant;
  onClick?: () => MaybePromise<void>;
}

withDefaults(defineProps<Props>(), {
  size: 'icon',
  variant: 'ghost',
});

defineSlots<{
  default?: () => VNode;
}>();
</script>

<template>
  <Button :variant :size :disabled :role :tabindex @click="() => onClick?.()">
    <component :is="icon" v-if="icon" />
    <slot v-else-if="$slots.default"></slot>
  </Button>
</template>
