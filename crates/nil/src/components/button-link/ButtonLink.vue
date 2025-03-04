<script setup lang="ts">
import type { VNode } from 'vue';
import { cn } from '@/lib/utils';
import { RouterLink } from 'vue-router';
import type { Route } from '@/router/types';
import type { Option } from '@tb-dev/utils';
import { createReusableTemplate } from '@vueuse/core';
import { Button, type ButtonSize, type ButtonVariant } from '../button';

type Props = {
  buttonClass?: string;
  disabled?: Option<boolean>;
  label?: string;
  size?: ButtonSize;
  to: Route;
  variant?: ButtonVariant;
};

withDefaults(defineProps<Props>(), {
  size: 'default',
  variant: 'ghost',
});

defineSlots<{ default?: () => VNode }>();

// eslint-disable-next-line @typescript-eslint/naming-convention
const [DefineTemplate, ReuseTemplate] = createReusableTemplate();
</script>

<template>
  <DefineTemplate>
    <Button :variant :size :disabled :class="cn('size-full', buttonClass)">
      <span v-if="label">{{ label }}</span>
      <slot v-else></slot>
    </Button>
  </DefineTemplate>

  <div v-if="disabled">
    <ReuseTemplate />
  </div>
  <RouterLink v-else :to="{ name: to }">
    <ReuseTemplate />
  </RouterLink>
</template>
