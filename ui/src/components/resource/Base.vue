<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { Option } from '@tb-dev/utils';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@tb-dev/vue-components';

const props = defineProps<{
  amount?: Option<number>;
  limit?: Option<number>;
  color: string;
  name: string;
}>();

function isOverflowing() {
  return (
    typeof props.amount === 'number' &&
    typeof props.limit === 'number' &&
    props.amount >= props.limit
  );
}
</script>

<template>
  <div class="flex items-center justify-start gap-1">
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger as-child>
          <div
            class="size-3 min-h-3 min-w-3 overflow-hidden rounded-full"
            :style="{ backgroundColor: color }"
          ></div>
        </TooltipTrigger>

        <TooltipContent>
          <div class="select-none">{{ name }}</div>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>

    <div :class="isOverflowing() ? 'text-red-400' : null">
      {{ amount ?? 0 }}
    </div>
  </div>
</template>
