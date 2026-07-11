<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { MaybePromise } from "@tb-dev/utils";
import type { ManeuverId } from "@tsukilabs/nil-bindings";
import type { CoordImpl } from "@/core/model/continent/coord";
import Maneuver from "@/scenes/game/war-room/root/Maneuver.vue";
import type { ManeuverImpl } from "@/core/model/military/maneuver";
import { Table, TableBody, TableHead, TableHeader, TableRow } from "@ui/table";

const props = defineProps<{
  maneuvers: ManeuverImpl[];
  warRoomOrigin: CoordImpl;
  onCancelManeuver: (id: ManeuverId) => MaybePromise<void>;
}>();

const { t } = useI18n();

function shouldShowManeuver(maneuver: ManeuverImpl) {
  return (
    maneuver.direction === "going" ||
    maneuver.origin.is(props.warRoomOrigin)
  );
}
</script>

<template>
  <Table class="min-w-max">
    <TableHeader>
      <TableRow>
        <TableHead></TableHead>
        <TableHead>{{ t("origin") }}</TableHead>
        <TableHead>{{ t("destination") }}</TableHead>
        <TableHead>{{ t("distance") }}</TableHead>
        <TableHead></TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <template v-for="maneuver of maneuvers" :key="maneuver.id">
        <Maneuver
          v-if="shouldShowManeuver(maneuver)"
          :maneuver
          :war-room-origin
          @cancel-maneuver="() => onCancelManeuver(maneuver.id)"
        />
      </template>
    </TableBody>
  </Table>
</template>
