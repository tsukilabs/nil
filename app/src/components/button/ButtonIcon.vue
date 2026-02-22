<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Button } from '@tb-dev/vue-components';
import type { Component, HTMLAttributes, VNode } from 'vue';

defineProps<{
  icon?: Component;
  role?: HTMLAttributes['role'];
  tabindex?: HTMLAttributes['tabindex'];
  onClick?: () => MaybePromise<void>;
}>();

defineSlots<{
  default?: () => VNode;
}>();
</script>

<template>
  <Button variant="ghost" size="icon" :role :tabindex @click="onClick">
    <component :is="icon" v-if="icon" />
    <slot v-else-if="$slots.default"></slot>
  </Button>
</template>
