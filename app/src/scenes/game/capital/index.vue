<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { useI18n } from "@tsukilabs/nil-i18n";
import { useCityLimit } from "@/composables/capital/useCityLimit";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { Table, TableBody, TableCell, TableHead, TableRow } from "@ui/table";

const { t } = useI18n();

const { stats } = NIL.world.refs();
const { player, cities } = NIL.player.refs();

const { cityLimit } = useCityLimit();
</script>

<template>
  <div class="game-layout">
    <Card v-if="stats && player" class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t("capital") }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full px-2 py-0 overflow-auto">
        <Table class="sm:max-w-max md:min-w-50">
          <TableBody>
            <TableRow>
              <TableHead>{{ t("capital.city-amount") }}</TableHead>
              <TableCell>{{ cities.length }}</TableCell>
            </TableRow>

            <TableRow>
              <TableHead>{{ t("capital.city-limit") }}</TableHead>
              <TableCell>{{ cityLimit }}</TableCell>
            </TableRow>

            <TableRow>
              <TableHead>{{ t("influence") }}</TableHead>
              <TableCell>{{ player.influence }}</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
