<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import ButtonIcon from '@/components/button/ButtonIcon.vue';
import type { Component, HTMLAttributes, VNode } from 'vue';

defineProps<{
  icon?: Component;
  hasUnread: boolean;
  role?: HTMLAttributes['role'];
  tabindex?: HTMLAttributes['tabindex'];
  onClick?: () => MaybePromise<void>;
}>();

defineSlots<{
  default?: () => VNode;
}>();
</script>

<template>
  <div class="relative">
    <ButtonIcon :icon :role :tabindex @click="onClick">
      <slot v-if="$slots.default"></slot>
    </ButtonIcon>

    <div
      v-if="hasUnread"
      class="absolute top-[4px] right-[4px] size-[10px] min-h-[10px] min-w-[10px] overflow-hidden rounded-full bg-red-500"
    >
    </div>
  </div>
</template>
