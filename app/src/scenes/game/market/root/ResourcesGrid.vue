<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import type { Option } from "@tb-dev/utils";
import type { MarketFee, Resources } from "@tsukilabs/nil-bindings";
import ResourcesGridItem from "@/scenes/game/market/root/ResourcesGridItem.vue";

const props = defineProps<{
  marketFee?: Option<MarketFee>;
  limitToAvailable?: boolean;
}>();

const resources = defineModel<Resources>({ required: true });
</script>

<template>
  <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
    <ResourcesGridItem v-model="resources.wood" kind="wood" :market-fee :limit-to-available />
    <ResourcesGridItem v-model="resources.stone" kind="stone" :market-fee :limit-to-available />
    <ResourcesGridItem v-model="resources.iron" kind="iron" :market-fee :limit-to-available />
    <ResourcesGridItem v-model="resources.food" kind="food" :market-fee :limit-to-available />
  </div>
</template>
