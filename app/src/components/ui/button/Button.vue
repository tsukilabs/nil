<script setup lang="ts">
import { cn } from '@/components/ui/utils';
import { buttonVariants } from '.';
import { Primitive } from 'reka-ui';
import type { ButtonVariants } from '.';
import type { HTMLAttributes } from 'vue';
import type { PrimitiveProps } from 'reka-ui';

interface Props extends PrimitiveProps {
  variant?: ButtonVariants['variant'];
  size?: ButtonVariants['size'];
  class?: HTMLAttributes['class'];
}

const props = withDefaults(defineProps<Props>(), {
  as: 'button',
});
</script>

<template>
  <Primitive
    data-slot="button"
    :data-variant="variant"
    :data-size="size"
    :as
    :as-child
    :class="cn(buttonVariants({ variant, size }), props.class)"
  >
    <slot />
  </Primitive>
</template>
