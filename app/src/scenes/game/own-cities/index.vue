<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from "@/router";
import { nextTick } from "vue";
import { useI18n } from "vue-i18n";
import { handleError } from "@/lib/error";
import { formatPercent } from "@/lib/intl";
import type { Coord } from "@tsukilabs/nil-bindings";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@ui/table";

const { t } = useI18n();

const { city: currentCity } = NIL.city.refs();
const { cities: playerCities } = NIL.player.refs();

async function goToCity(coord: Coord) {
  try {
    await nextTick();
    const city = playerCities.value.find((it) => {
      return it.coord.is(coord);
    });

    if (city && (!currentCity.value || !city.coord.is(currentCity.value.coord))) {
      await NIL.city.setCoord(city.coord);
      await go("city");
    }
  }
  catch (err) {
    handleError(err);
  }
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t("city", 2) }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="relative size-full overflow-auto px-2 py-0">
        <Table v-if="playerCities.length > 0" class="min-w-max">
          <TableHeader>
            <TableRow class="hover:bg-card">
              <TableHead>{{ t("coordinate", 2) }}</TableHead>
              <TableHead>{{ t("name") }}</TableHead>
              <TableHead>{{ t("score") }}</TableHead>
              <TableHead>{{ t("stability") }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow
              v-for="city of playerCities"
              :key="city.coord.id"
              role="link"
              tabindex="0"
              class="cursor-pointer"
              @click.stop="() => void goToCity(city.coord)"
              @keydown.enter.stop="() => void goToCity(city.coord)"
              @keydown.space.stop="() => void goToCity(city.coord)"
            >
              <TableCell>
                {{ city.coord.id }}
              </TableCell>
              <TableCell>
                {{ city.name }}
              </TableCell>
              <TableCell>
                {{ city.score }}
              </TableCell>
              <TableCell>
                {{ formatPercent(city.stability) }}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
