<script setup lang="ts">
import { cn } from '@/lib/utils';
import type { VNode } from 'vue';
import { Label as BaseLabel } from '@/components/base/label';

type Props = {
  class?: string;
};

const props = defineProps<Props>();

defineSlots<{ default: () => VNode }>();
</script>

<template>
  <BaseLabel :class="cn('flex w-full flex-col gap-2 font-normal', props.class)">
    <slot></slot>
  </BaseLabel>
</template>
