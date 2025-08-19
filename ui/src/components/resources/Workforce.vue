<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Base from './Base.vue';

defineProps<{
  amount?: Option<number>;
  limit?: Option<number>;
  alwaysLiteral?: boolean;
}>();
</script>

<template>
  <Base :amount :limit color="oklch(0.38 0.0493 83.65)" :always-literal />
</template>
