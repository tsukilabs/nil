<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { onKeyDown } from "@vueuse/core";
import type { Option } from "@tb-dev/utils";
import Loading from "@/components/Loading.vue";
import { throttle } from "es-toolkit/function";
import { useRouteParams } from "@vueuse/router";
import Food from "@/components/resources/Food.vue";
import Iron from "@/components/resources/Iron.vue";
import Wood from "@/components/resources/Wood.vue";
import Stone from "@/components/resources/Stone.vue";
import type { ManeuverId } from "@tsukilabs/nil-bindings";
import { useManeuver } from "@/composables/military/useManeuver";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@ui/table";

const { t } = useI18n();

const id = useRouteParams<Option<ManeuverId>>("id", null);
const { army, maneuver, loading, loadManeuver } = useManeuver(id);

if (__DESKTOP__) {
  onKeyDown("F5", throttle(loadManeuver, 1000));
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t("maneuver") }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="relative size-full overflow-auto px-2 py-0">
        <Loading v-if="loading" />
        <div v-else-if="maneuver" class="w-full min-w-max flex flex-col gap-4">
          <Table class="sm:max-w-max">
            <TableBody>
              <TableRow>
                <TableHead>{{ t("kind") }}</TableHead>
                <TableCell>{{ t(maneuver.kind) }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t("origin") }}</TableHead>
                <TableCell
                  role="link"
                  tabindex="0"
                  class="cursor-pointer"
                  @click.stop="() => maneuver?.origin.goToProfile()"
                  @keydown.enter.stop="() => maneuver?.origin.goToProfile()"
                  @keydown.space.stop="() => maneuver?.origin.goToProfile()"
                >
                  {{ maneuver.origin.format() }}
                </TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t("destination") }}</TableHead>
                <TableCell
                  role="link"
                  tabindex="0"
                  class="cursor-pointer"
                  @click.stop="() => maneuver?.destination.goToProfile()"
                  @keydown.enter.stop="() => maneuver?.destination.goToProfile()"
                  @keydown.space.stop="() => maneuver?.destination.goToProfile()"
                >
                  {{ maneuver.destination.format() }}
                </TableCell>
              </TableRow>

              <TableRow v-if="maneuver.state.kind === 'pending'">
                <TableHead>{{ t("distance") }}</TableHead>
                <TableCell>{{ maneuver.state.distance }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>

          <Table v-if="army && !army.isEmpty()" class="sm:max-w-max">
            <TableHeader>
              <TableRow class="hover:bg-card">
                <TableHead>{{ t("pikeman") }}</TableHead>
                <TableHead>{{ t("swordsman") }}</TableHead>
                <TableHead>{{ t("axeman") }}</TableHead>
                <TableHead>{{ t("archer") }}</TableHead>
                <TableHead>{{ t("light-cavalry") }}</TableHead>
                <TableHead>{{ t("heavy-cavalry") }}</TableHead>
                <TableHead>{{ t("ram") }}</TableHead>
              </TableRow>
            </TableHeader>

            <TableBody>
              <TableRow class="hover:bg-card">
                <TableCell>{{ army.pikeman.size }}</TableCell>
                <TableCell>{{ army.swordsman.size }}</TableCell>
                <TableCell>{{ army.axeman.size }}</TableCell>
                <TableCell>{{ army.archer.size }}</TableCell>
                <TableCell>{{ army.lightCavalry.size }}</TableCell>
                <TableCell>{{ army.heavyCavalry.size }}</TableCell>
                <TableCell>{{ army.ram.size }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>

          <Table
            v-if="maneuver.hauledResources && maneuver.hasHauledResources()"
            class="sm:max-w-max"
          >
            <TableBody>
              <TableRow class="hover:bg-card">
                <TableHead>{{ t("haul") }}</TableHead>
                <TableCell>
                  <div class="w-max max-w-max flex items-center gap-4">
                    <Wood :amount="maneuver.hauledResources.resources.wood" />
                    <Stone :amount="maneuver.hauledResources.resources.stone" />
                    <Iron :amount="maneuver.hauledResources.resources.iron" />
                    <Food :amount="maneuver.hauledResources.resources.food" />
                  </div>
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
