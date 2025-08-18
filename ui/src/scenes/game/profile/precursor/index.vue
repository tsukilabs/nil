<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRouteParams } from '@vueuse/router';
import VillageTable from '@/components/profile/VillageTable.vue';
import { usePublicVillages } from '@/composables/village/usePublicVillages';
import { usePublicPrecursor } from '@/composables/npc/precursor/usePublicPrecursor';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const id = useRouteParams<Option<PrecursorId>>('id', null);
const { precursor, coords, loading } = usePublicPrecursor(id);
const { villages } = usePublicVillages(coords);
</script>

<template>
  <div class="game-layout">
    <Card class="size-full overflow-x-hidden overflow-y-auto">
      <CardHeader v-if="precursor && !loading">
        <CardTitle>
          <span>{{ precursor.id }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0 relative size-full">
        <div class="flex w-full min-w-max flex-col gap-4">
          <Table class="sm:max-w-max md:min-w-50">
            <TableBody>
              <TableRow>
                <TableHead>{{ t('point', 2) }}</TableHead>
                <TableCell>???</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('rank') }}</TableHead>
                <TableCell>???</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('type') }}</TableHead>
                <TableCell>{{ t('precursor') }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>

          <VillageTable :villages class="sm:max-w-max md:min-w-50" />
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
