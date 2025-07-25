<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { VNode } from 'vue';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@tb-dev/vue-components';

defineProps<{ label: string; }>();

defineSlots<{ default: () => VNode; }>();
</script>

<template>
  <TooltipProvider :delay-duration="700">
    <Tooltip>
      <TooltipTrigger as-child>
        <slot></slot>
      </TooltipTrigger>
      <TooltipContent>
        <span>{{ label }}</span>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>
