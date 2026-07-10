<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { ManeuverImpl } from "@/core/model/military/maneuver";
import { ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon } from "@lucide/vue";

defineProps<{
  maneuver: ManeuverImpl;
}>();

const { player } = NIL.player.refs();
</script>

<!-- eslint-disable vue/no-root-v-if -->
<template>
  <template v-if="maneuver.direction === 'going'">
    <span v-if="player?.owns(maneuver.origin)">
      <ChevronRightIcon v-if="maneuver.kind === 'attack'" class="size-4 text-green-500" />
      <ChevronRightIcon v-if="maneuver.kind === 'support'" class="size-4 text-blue-500" />
    </span>
    <span v-else-if="player?.owns(maneuver.destination)">
      <ChevronLeftIcon v-if="maneuver.kind === 'attack'" class="size-4 text-red-500" />
      <ChevronLeftIcon v-else-if="maneuver.kind === 'support'" class="size-4 text-blue-500" />
    </span>
  </template>
  <span v-else-if="maneuver.direction === 'returning'">
    <ChevronDownIcon v-if="maneuver.kind === 'attack'" class="size-4" />
    <ChevronDownIcon v-else-if="maneuver.kind === 'support'" class="size-4" />
  </span>
</template>
