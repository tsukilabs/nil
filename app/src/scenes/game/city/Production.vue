<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from "@/router";
import { useI18n } from "vue-i18n";
import { Table, TableBody, TableCell, TableRow } from "@ui/table";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";

const { t } = useI18n();

const { production } = NIL.city.refs();
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
        <span>{{ t("production") }}</span>
      </CardTitle>
    </CardHeader>

    <CardContent class="px-2 py-0">
      <Table>
        <TableBody>
          <TableRow
            v-if="production?.wood"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click.stop="() => go('sawmill')"
            @keydown.enter.stop="() => go('sawmill')"
            @keydown.space.stop="() => go('sawmill')"
          >
            <TableCell>{{ t("wood") }}</TableCell>
            <TableCell>{{ t("x-per-turn", [production.wood]) }}</TableCell>
          </TableRow>

          <TableRow
            v-if="production?.stone"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click.stop="() => go('quarry')"
            @keydown.enter.stop="() => go('quarry')"
            @keydown.space.stop="() => go('quarry')"
          >
            <TableCell>{{ t("stone") }}</TableCell>
            <TableCell>{{ t("x-per-turn", [production.stone]) }}</TableCell>
          </TableRow>

          <TableRow
            v-if="production?.iron"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click.stop="() => go('iron-mine')"
            @keydown.enter.stop="() => go('iron-mine')"
            @keydown.space.stop="() => go('iron-mine')"
          >
            <TableCell>{{ t("iron") }}</TableCell>
            <TableCell>{{ t("x-per-turn", [production.iron]) }}</TableCell>
          </TableRow>

          <TableRow
            v-if="production?.food"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click.stop="() => go('farm')"
            @keydown.enter.stop="() => go('farm')"
            @keydown.space.stop="() => go('farm')"
          >
            <TableCell>{{ t("food") }}</TableCell>
            <TableCell>{{ t("x-per-turn", [production.food]) }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
