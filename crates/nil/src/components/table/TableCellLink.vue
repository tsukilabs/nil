<script setup lang="ts">
import { Link } from '../link';
import type { VNode } from 'vue';
import { cn } from '@/lib/utils';
import { TableCell } from '../base/table';
import type { Route } from '@/router/types';

type Props = {
  label?: string;
  linkClass?: string;
  to: Route;
};

defineProps<Props>();

defineSlots<{ default?: () => VNode }>();
</script>

<template>
  <TableCell>
    <Link :to :label :class="cn('flex w-full', linkClass)">
      <slot></slot>
    </Link>
  </TableCell>
</template>
