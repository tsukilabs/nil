<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { onKeyDown } from '@tb-dev/vue';
import { throttle } from 'es-toolkit/function';
import { useRouteParams } from '@vueuse/router';
import CityTable from '@/components/CityTable.vue';
import type { PrecursorId } from '@/types/core/npc/precursor';
import { Card, CardContent, CardHeader, CardTitle } from '@ui/card';
import { usePublicCities } from '@/composables/city/usePublicCities';
import { usePublicPrecursor } from '@/composables/npc/usePublicPrecursor';
import { Table, TableBody, TableCell, TableHead, TableRow } from '@ui/table';

const { t } = useI18n();

const id = useRouteParams<Option<PrecursorId>>('id', null);
const { precursor, coords, load } = usePublicPrecursor(id);
const { cities } = usePublicCities(coords);

if (__DESKTOP__) {
  onKeyDown('F5', throttle(load, 1000));
}
</script>

<template>
  <div class="game-layout">
    <Card v-if="precursor" class="size-full overflow-hidden">
      <CardHeader>
        <CardTitle>
          <span>{{ precursor.id }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full relative px-2 py-0 overflow-auto">
        <div class="flex w-full min-w-max flex-col gap-4">
          <Table class="sm:max-w-max md:min-w-50">
            <TableBody>
              <TableRow>
                <TableHead>{{ t('point', 2) }}</TableHead>
                <TableCell>{{ precursor.formatScore() }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('rank') }}</TableHead>
                <TableCell>{{ precursor.formatRank() }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('type') }}</TableHead>
                <TableCell>{{ t('precursor') }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>

          <CityTable
            :cities
            sortable
            default-sort-mode="name"
            default-sort-order="asc"
            class="sm:max-w-max md:min-w-50 pb-8"
          />
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
:deep(table :where(th, td)) {
  padding-right: 2rem;
}
</style>
