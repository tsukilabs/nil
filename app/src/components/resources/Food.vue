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
    color="oklch(0.7 0.1751 133.41)"
    :amount
    :limit
    :always-literal
    :icon-class
    :text-class
  />
</template>
