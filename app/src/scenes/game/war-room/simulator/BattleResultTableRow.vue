<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { formatInt } from '@/lib/intl';
import { TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

defineProps<{
  head: string;
  personnel: ArmyPersonnel;
}>();
</script>

<template>
  <TableRow>
    <TableHead>{{ head }}</TableHead>
    <TableCell>{{ formatInt(personnel.pikeman.size) }}</TableCell>
    <TableCell>{{ formatInt(personnel.swordsman.size) }}</TableCell>
    <TableCell>{{ formatInt(personnel.axeman.size) }}</TableCell>
    <TableCell>{{ formatInt(personnel.archer.size) }}</TableCell>
    <TableCell>{{ formatInt(personnel.lightCavalry.size) }}</TableCell>
    <TableCell>{{ formatInt(personnel.heavyCavalry.size) }}</TableCell>
    <TableCell>{{ formatInt(personnel.ram.size) }}</TableCell>
  </TableRow>
</template>
