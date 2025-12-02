<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Base from './Base.vue';
import type { ClassValue } from '@tb-dev/vue-components';

defineProps<{
  amount?: Option<number>;
  limit?: Option<number>;
  alwaysLiteral?: boolean;
  iconClass?: ClassValue;
  textClass?: ClassValue;
}>();
</script>

<template>
  <Base
    color="oklch(0.77 0.1332 78.35)"
    :amount
    :limit
    :always-literal
    :icon-class
    :text-class
  />
</template>
