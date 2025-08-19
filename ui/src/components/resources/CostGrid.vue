<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Wood from './Wood.vue';
import Iron from './Iron.vue';
import Food from './Food.vue';
import Stone from './Stone.vue';
import Workforce from './Workforce.vue';

defineProps<{
  resources: Resources;
  maintenance: number;
  workforce: number;
  limit?: Option<Partial<Resources>>;
  alwaysLiteral?: boolean;
}>();
</script>

<template>
  <div class="grid grid-cols-5 items-center justify-start gap-4">
    <Wood :amount="resources.wood" :limit="limit?.wood" :always-literal />
    <Stone :amount="resources.stone" :limit="limit?.stone" :always-literal />
    <Iron :amount="resources.iron" :limit="limit?.iron" :always-literal />
    <Food :amount="maintenance" :always-literal />
    <Workforce :amount="workforce" :always-literal />
  </div>
</template>
