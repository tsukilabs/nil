<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { TableCell, TableRow } from "@ui/table";
import type { Coord } from "@tsukilabs/nil-bindings";
import { ChevronLeftIcon, ChevronRightIcon } from "@lucide/vue";
import type { ManeuverImpl } from "@/core/model/military/maneuver";

defineProps<{
  maneuver: ManeuverImpl;
}>();

const { t } = useI18n();

const { coord: currentCoord } = NIL.city.refs();

const distanceIntl = new Intl.NumberFormat(undefined, {
  style: "decimal",
  minimumFractionDigits: 0,
  maximumFractionDigits: 2,
  roundingMode: "floor",
  notation: "standard",
  useGrouping: "auto",
  localeMatcher: "best fit",
});

function getCoordCellClass(coord: Coord) {
  return currentCoord.value?.is(coord) ? "font-bold" : null;
}

function formatDistance(distance: number) {
  return distanceIntl.format(distance);
}
</script>

<template>
  <TableRow
    role="link"
    tabindex="0"
    class="cursor-pointer"
    @click.stop="() => maneuver.goToManeuverScene()"
    @keydown.enter.stop="() => maneuver.goToManeuverScene()"
    @keydown.space.stop="() => maneuver.goToManeuverScene()"
  >
    <TableCell class="w-[1%] pl-0 pr-2">
      <span v-if="maneuver.direction === 'returning'">
        <ChevronLeftIcon class="size-4 text-red-500" />
      </span>
      <span v-else>
        <ChevronRightIcon class="size-4 text-green-500" />
      </span>
    </TableCell>

    <TableCell>
      <span v-if="maneuver.kind === 'attack'">{{ t("attack-noun") }}</span>
      <span v-else-if="maneuver.kind === 'support'">{{ t("support-noun") }}</span>
      <span v-else>{{ t(maneuver.kind) }}</span>
    </TableCell>

    <TableCell
      role="link"
      tabindex="0"
      :class="getCoordCellClass(maneuver.origin)"
      @click.stop="() => maneuver.origin.goToProfile()"
      @keydown.enter.stop="() => maneuver.origin.goToProfile()"
      @keydown.space.stop="() => maneuver.origin.goToProfile()"
    >
      {{ maneuver.origin.format() }}
    </TableCell>

    <TableCell
      role="link"
      tabindex="0"
      :class="getCoordCellClass(maneuver.destination)"
      @click.stop="() => maneuver.destination.goToProfile()"
      @keydown.enter.stop="() => maneuver.destination.goToProfile()"
      @keydown.space.stop="() => maneuver.destination.goToProfile()"
    >
      {{ maneuver.destination.format() }}
    </TableCell>

    <TableCell>
      <span>{{ formatDistance(maneuver.getPendingDistance()) }}</span>
    </TableCell>
  </TableRow>
</template>
