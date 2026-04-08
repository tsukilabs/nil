<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { Coord } from '@/types/core/continent';
import { ChevronLeftIcon, ChevronRightIcon } from '@lucide/vue';
import type { ManeuverImpl } from '@/core/model/military/maneuver';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@ui/table';

defineProps<{
  maneuvers: ManeuverImpl[];
}>();

const { t } = useI18n();

const { coord: currentCoord } = NIL.city.refs();

function getCoordCellClass(coord: Coord) {
  return currentCoord.value?.is(coord) ? 'font-bold' : null;
}
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead></TableHead>
        <TableHead>{{ t('kind') }}</TableHead>
        <TableHead>{{ t('origin') }}</TableHead>
        <TableHead>{{ t('destination') }}</TableHead>
        <TableHead>{{ t('distance') }}</TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <TableRow v-for="maneuver of maneuvers" :key="maneuver.id" class="cursor-pointer">
        <TableCell class="w-[1%] pl-0 pr-2">
          <span v-if="maneuver.direction === 'returning'">
            <ChevronLeftIcon class="size-4 text-red-500" />
          </span>
          <span v-else>
            <ChevronRightIcon class="size-4 text-green-500" />
          </span>
        </TableCell>

        <TableCell>
          <span v-if="maneuver.kind === 'attack'">{{ t('attack-noun') }}</span>
          <span v-else-if="maneuver.kind === 'support'">{{ t('support-noun') }}</span>
          <span v-else>{{ t(maneuver.kind) }}</span>
        </TableCell>

        <TableCell
          role="link"
          tabindex="0"
          :class="getCoordCellClass(maneuver.origin)"
          @click.stop="() => maneuver.origin.goToProfile()"
          @keydown.enter="() => maneuver.origin.goToProfile()"
          @keydown.space="() => maneuver.origin.goToProfile()"
        >
          {{ maneuver.origin.format() }}
        </TableCell>

        <TableCell
          role="link"
          tabindex="0"
          :class="getCoordCellClass(maneuver.destination)"
          @click.stop="() => maneuver.destination.goToProfile()"
          @keydown.enter="() => maneuver.destination.goToProfile()"
          @keydown.space="() => maneuver.destination.goToProfile()"
        >
          {{ maneuver.destination.format() }}
        </TableCell>

        <TableCell>
          <span>{{ maneuver.getPendingDistance() }}</span>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
