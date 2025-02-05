<script setup lang="ts">
import type { VNode } from 'vue';
import { Table, TableBody, TableHeader } from '../base/table';

defineSlots<{
  default: () => VNode;
  header?: () => VNode;
}>();
</script>

<template>
  <Table>
    <TableHeader v-if="$slots.header">
      <slot name="header"></slot>
    </TableHeader>
    <TableBody>
      <slot></slot>
    </TableBody>
  </Table>
</template>
