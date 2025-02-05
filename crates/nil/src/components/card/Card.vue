<script setup lang="ts">
import type { VNode } from 'vue';
import { cn } from '@/lib/utils';
import { toPixel } from '@tb-dev/utils';
import { ScrollArea as UiScrollArea } from '@/components/base/scroll-area';
import {
  Card as BaseCard,
  CardContent as BaseCardContent,
  CardDescription as BaseCardDescription,
  CardHeader as BaseCardHeader,
  CardTitle as BaseCardTitle,
} from '@/components/base/card';

interface Props {
  contentClass?: string;
  descriptionClass?: string;
  headerClass?: string;
  scrollAreaClass?: string;
  scrollAreaHeight?: number | string;
  titleClass?: string;
}

withDefaults(defineProps<Props>(), {
  scrollAreaHeight: 'auto',
});

defineSlots<{
  default: () => VNode;
  description?: () => VNode;
  title?: () => VNode;
}>();
</script>

<template>
  <BaseCard>
    <BaseCardHeader v-if="$slots.title" :class="headerClass">
      <BaseCardTitle :class="titleClass">
        <slot name="title"></slot>
      </BaseCardTitle>
      <BaseCardDescription v-if="$slots.description" :class="descriptionClass">
        <slot name="description"></slot>
      </BaseCardDescription>
    </BaseCardHeader>
    <BaseCardContent :class="cn('p-0', contentClass)">
      <UiScrollArea
        v-if="scrollAreaHeight && scrollAreaHeight !== 'auto'"
        :class="scrollAreaClass"
        :style="{ height: toPixel(scrollAreaHeight) }"
      >
        <slot></slot>
      </UiScrollArea>
      <slot v-else></slot>
    </BaseCardContent>
  </BaseCard>
</template>
