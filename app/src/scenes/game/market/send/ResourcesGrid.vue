<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import type { MarketFee, Resources } from "@tsukilabs/nil-bindings";
import ResourcesGridItem from "@/scenes/game/market/send/ResourcesGridItem.vue";

const props = defineProps<{
  marketFee: MarketFee;
}>();

const resources = defineModel<Resources>({ required: true });
</script>

<template>
  <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
    <ResourcesGridItem v-model="resources.wood" kind="wood" :market-fee />
    <ResourcesGridItem v-model="resources.stone" kind="stone" :market-fee />
    <ResourcesGridItem v-model="resources.iron" kind="iron" :market-fee />
    <ResourcesGridItem v-model="resources.food" kind="food" :market-fee />
  </div>
</template>
