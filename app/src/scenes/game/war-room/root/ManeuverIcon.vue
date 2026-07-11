<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { CoordImpl } from "@/core/model/continent/coord";
import type { ManeuverImpl } from "@/core/model/military/maneuver";
import { ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon } from "@lucide/vue";

defineProps<{
  maneuver: ManeuverImpl;
  warRoomOrigin: CoordImpl;
}>();
</script>

<!-- eslint-disable vue/no-root-v-if -->
<template>
  <div>
    <span v-if="maneuver.direction === 'going' && maneuver.origin.is(warRoomOrigin)">
      <ChevronRightIcon v-if="maneuver.kind === 'attack'" class="size-4 text-green-500" />
      <ChevronRightIcon v-if="maneuver.kind === 'support'" class="size-4 text-blue-500" />
    </span>
    <span v-else-if="maneuver.direction === 'going' && maneuver.destination.is(warRoomOrigin)">
      <ChevronLeftIcon v-if="maneuver.kind === 'attack'" class="size-4 text-red-500" />
      <ChevronLeftIcon v-else-if="maneuver.kind === 'support'" class="size-4 text-blue-500" />
    </span>
    <span v-else-if="maneuver.direction === 'returning' && maneuver.origin.is(warRoomOrigin)">
      <ChevronDownIcon v-if="maneuver.kind === 'attack'" class="size-4" />
      <ChevronDownIcon v-else-if="maneuver.kind === 'support'" class="size-4" />
    </span>
  </div>
</template>
