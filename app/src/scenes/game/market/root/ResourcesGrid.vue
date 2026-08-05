<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import type { Option } from "@tb-dev/utils";
import type { Resources } from "@tsukilabs/nil-bindings";
import type { MarketImpl } from "@/core/model/market/market";
import ResourcesGridItem from "@/scenes/game/market/root/ResourcesGridItem.vue";

const props = defineProps<{
  market?: Option<MarketImpl>;
  limitToAvailable?: boolean;
}>();

const resources = defineModel<Resources>({ required: true });
</script>

<template>
  <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
    <ResourcesGridItem v-model="resources.wood" kind="wood" :market :limit-to-available />
    <ResourcesGridItem v-model="resources.stone" kind="stone" :market :limit-to-available />
    <ResourcesGridItem v-model="resources.iron" kind="iron" :market :limit-to-available />
    <ResourcesGridItem v-model="resources.food" kind="food" :market :limit-to-available />
  </div>
</template>
