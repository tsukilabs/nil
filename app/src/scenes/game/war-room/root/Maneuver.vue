<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { XIcon } from "@lucide/vue";
import { asyncComputed } from "@tb-dev/vue";
import { TableCell, TableRow } from "@ui/table";
import type { MaybePromise } from "@tb-dev/utils";
import type { Coord } from "@tsukilabs/nil-bindings";
import type { CoordImpl } from "@/core/model/continent/coord";
import type { ManeuverImpl } from "@/core/model/military/maneuver";
import ManeuverIcon from "@/scenes/game/war-room/root/ManeuverIcon.vue";

const props = defineProps<{
  maneuver: ManeuverImpl;
  warRoomOrigin: CoordImpl;
  onCancelManeuver: () => MaybePromise<void>;
}>();

const { id: player } = NIL.player.refs();

const armyOwner = asyncComputed(null, () => props.maneuver.getArmyOwner());
const cities = asyncComputed(null, () => props.maneuver.getCities());

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
  return props.warRoomOrigin.is(coord) ? "font-bold" : null;
}

function formatDistance(distance: number) {
  return distanceIntl.format(distance);
}

function isArmyOwnedByCurrentPlayer() {
  return (
    player.value &&
    armyOwner.value?.kind === "player" &&
    armyOwner.value.id === player.value
  );
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
      <ManeuverIcon :maneuver :war-room-origin />
    </TableCell>

    <TableCell
      role="link"
      tabindex="0"
      :class="getCoordCellClass(maneuver.origin)"
      @click.stop="() => maneuver.origin.goToProfile()"
      @keydown.enter.stop="() => maneuver.origin.goToProfile()"
      @keydown.space.stop="() => maneuver.origin.goToProfile()"
    >
      <span v-if="cities?.origin">{{ cities.origin.formatNameWithCoord() }}</span>
      <span v-else>{{ maneuver.origin.format() }}</span>
    </TableCell>

    <TableCell
      role="link"
      tabindex="0"
      :class="getCoordCellClass(maneuver.destination)"
      @click.stop="() => maneuver.destination.goToProfile()"
      @keydown.enter.stop="() => maneuver.destination.goToProfile()"
      @keydown.space.stop="() => maneuver.destination.goToProfile()"
    >
      <span v-if="cities?.destination">{{ cities.destination.formatNameWithCoord() }}</span>
      <span v-else>{{ maneuver.destination.format() }}</span>
    </TableCell>

    <TableCell>
      <span>{{ formatDistance(maneuver.getPendingDistance()) }}</span>
    </TableCell>

    <TableCell @click.stop.prevent>
      <XIcon
        v-if="maneuver.direction === 'going' && isArmyOwnedByCurrentPlayer()"
        class="size-4"
        @click.stop="onCancelManeuver"
      />
    </TableCell>
  </TableRow>
</template>
