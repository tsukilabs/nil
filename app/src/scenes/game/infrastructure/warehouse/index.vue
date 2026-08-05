<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { formatInt } from "@/lib/intl";
import { useI18n } from "@tsukilabs/nil-i18n";
import Food from "@/components/resources/Food.vue";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { useWarehouse } from "@/composables/infrastructure/useBuilding";
import { useStorageStats } from "@/composables/infrastructure/useStorageStats";
import { Table, TableBody, TableCell, TableFooter, TableHead, TableHeader, TableRow } from "@ui/table";

const { t } = useI18n();

const warehouse = useWarehouse();
const { level, stats } = useStorageStats(warehouse);
</script>

<template>
  <div class="game-layout">
    <Card v-if="warehouse" class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ `${t("warehouse")} (${t("level-x", [level.current])})` }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full px-2 py-0 overflow-auto">
        <Table v-if="stats.current" class="min-w-max">
          <TableHeader>
            <TableRow class="bg-card hover:bg-card">
              <TableHead></TableHead>
              <TableHead>{{ t("storage.capacity") }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow>
              <TableCell class="w-72">
                <span>{{ t("storage.current-capacity") }}</span>
              </TableCell>
              <TableCell>
                <span>{{ formatInt(stats.current.capacity) }}</span>
              </TableCell>
            </TableRow>

            <TableRow v-if="stats.next && !level.isMax">
              <TableCell class="w-72">
                <span>{{ t("storage.capacity-on-level-x", [level.next]) }}</span>
              </TableCell>
              <TableCell>
                <span>{{ formatInt(stats.next.capacity) }}</span>
              </TableCell>
            </TableRow>
          </TableBody>

          <TableFooter>
            <TableRow class="bg-card hover:bg-card">
              <TableCell colspan="2">
                <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                  <div>{{ `${t("maintenance")}:` }}</div>
                  <Food :amount="warehouse.getMaintenance()" />
                </div>
              </TableCell>
            </TableRow>
          </TableFooter>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
