<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Wood from './Wood.vue';
import Iron from './Iron.vue';
import Food from './Food.vue';
import Stone from './Stone.vue';
import Workforce from './Workforce.vue';
import type { ClassValue } from '@tb-dev/vue-components';

defineProps<{
  resources: Resources;
  maintenance: number;
  workforce: number;
  limit?: Option<Partial<Resources>>;
  alwaysLiteral?: boolean;
  iconClass?: ClassValue;
  textClass?: ClassValue;
}>();
</script>

<template>
  <div class="grid grid-cols-5 items-center justify-start gap-4">
    <Wood
      :amount="resources.wood"
      :limit="limit?.wood"
      :always-literal
      :icon-class
      :text-class
    />
    <Stone
      :amount="resources.stone"
      :limit="limit?.stone"
      :always-literal
      :icon-class
      :text-class
    />
    <Iron
      :amount="resources.iron"
      :limit="limit?.iron"
      :always-literal
      :icon-class
      :text-class
    />
    <Food
      :amount="maintenance"
      :always-literal
      :icon-class
      :text-class
    />
    <Workforce
      :amount="workforce"
      :always-literal
      :icon-class
      :text-class
    />
  </div>
</template>
