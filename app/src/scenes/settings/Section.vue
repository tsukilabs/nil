<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { VNode } from 'vue';

defineProps<{
  title: string;
  titleId: string;
}>();

defineSlots<{
  default: () => VNode;
}>();
</script>

<template>
  <section class="flex flex-col gap-4">
    <h4 :id="titleId" class="text-lg">{{ title }}</h4>
    <slot></slot>
  </section>
</template>
