<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { TableCell, TableHead, TableRow } from '@ui/table';
import type { ArmyPersonnel } from '@/types/core/military/army';

defineProps<{
  head: string;
  personnel: ArmyPersonnel;
}>();
</script>

<template>
  <TableRow>
    <TableHead>{{ head }}</TableHead>
    <TableCell>{{ personnel.pikeman.size }}</TableCell>
    <TableCell>{{ personnel.swordsman.size }}</TableCell>
    <TableCell>{{ personnel.axeman.size }}</TableCell>
    <TableCell>{{ personnel.archer.size }}</TableCell>
    <TableCell>{{ personnel.lightCavalry.size }}</TableCell>
    <TableCell>{{ personnel.heavyCavalry.size }}</TableCell>
    <TableCell>{{ personnel.ram.size }}</TableCell>
  </TableRow>
</template>
