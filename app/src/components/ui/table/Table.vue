<script setup lang="ts">
import { cn } from '@/components/ui/utils';
import { type Option, toPixel } from '@tb-dev/utils';
import { computed, type CSSProperties, type HTMLAttributes, type VNode } from 'vue';

const props = defineProps<{
  class?: HTMLAttributes['class'];
  height?: number | string;
  style?: HTMLAttributes['style'];
  tableClass?: HTMLAttributes['class'];
  tableStyle?: HTMLAttributes['style'];
  width?: number | string;
}>();

defineSlots<{ default: () => VNode; }>();

const containerHeight = computed<Option<CSSProperties>>(() => {
  return props.height ? { maxHeight: toPixel(props.height) } : null;
});

const containerWidth = computed<Option<CSSProperties>>(() => {
  return props.width ? { maxWidth: toPixel(props.width) } : null;
});
</script>

<template>
  <div
    data-slot="table-container"
    :style="[containerHeight, containerWidth, props.style]"
    :class="cn(
      'relative w-full',
      containerWidth ? 'overflow-x-auto' : 'overflow-x-hidden',
      containerHeight ? 'overflow-y-auto' : 'overflow-y-hidden',
      props.class,
    )"
  >
    <table
      data-slot="table"
      :style="props.tableStyle"
      :class="cn('w-full caption-bottom text-sm', props.tableClass)"
    >
      <slot></slot>
    </table>
  </div>
</template>
