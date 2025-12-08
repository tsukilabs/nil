<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { VNode } from 'vue';
import { Button } from '@tb-dev/vue-components';

defineProps<{
  hasUnread: boolean;
}>();

defineSlots<{
  default: () => VNode;
}>();
</script>

<template>
  <div class="relative">
    <Button variant="ghost" size="icon">
      <slot></slot>
    </Button>

    <div
      v-if="hasUnread"
      class="absolute top-[4px] right-[4px] size-[10px] min-h-[10px] min-w-[10px] overflow-hidden rounded-full bg-red-500"
    >
    </div>
  </div>
</template>
