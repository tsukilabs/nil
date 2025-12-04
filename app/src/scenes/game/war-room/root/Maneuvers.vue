<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { ManeuverImpl } from '@/core/model/military/maneuver';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

defineProps<{
  maneuvers: ManeuverImpl[];
}>();
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead>Kind</TableHead>
        <TableHead>Destination</TableHead>
        <TableHead>Distance</TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <TableRow v-for="maneuver of maneuvers" :key="maneuver.id">
        <TableCell>{{ maneuver.kind }}</TableCell>
        <TableCell>{{ maneuver.destination.format() }}</TableCell>
        <TableCell>{{ maneuver.getPendingDistance() }}</TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
