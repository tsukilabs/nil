<script setup lang="ts">
import type { NumberFieldRootEmits, NumberFieldRootProps } from 'reka-ui';
import type { HTMLAttributes } from 'vue';
import { reactiveOmit } from '@vueuse/core';
import { NumberFieldRoot, useForwardPropsEmits } from 'reka-ui';
import { cn } from '@/components/ui/utils';

const props = defineProps<NumberFieldRootProps & { class?: HTMLAttributes['class']; }>();
const emits = defineEmits<NumberFieldRootEmits>();

const delegatedProps = reactiveOmit(props, 'class', 'formatOptions');

const forwarded = useForwardPropsEmits(delegatedProps, emits);
</script>

<template>
  <NumberFieldRoot
    #default="slotProps"
    v-bind="forwarded"
    :format-options="{ useGrouping: false, ...props.formatOptions ?? {} }"
    :class="cn('grid gap-1.5', props.class)"
  >
    <slot v-bind="slotProps" />
  </NumberFieldRoot>
</template>
