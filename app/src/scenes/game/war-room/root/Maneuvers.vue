<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Maneuver from "./Maneuver.vue";
import type { ManeuverImpl } from "@/core/model/military/maneuver";
import { Table, TableBody, TableHead, TableHeader, TableRow } from "@ui/table";

defineProps<{
  maneuvers: ManeuverImpl[];
}>();

const { t } = useI18n();
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead></TableHead>
        <TableHead>{{ t("kind") }}</TableHead>
        <TableHead>{{ t("origin") }}</TableHead>
        <TableHead>{{ t("destination") }}</TableHead>
        <TableHead>{{ t("distance") }}</TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <Maneuver v-for="maneuver of maneuvers" :key="maneuver.id" :maneuver />
    </TableBody>
  </Table>
</template>
