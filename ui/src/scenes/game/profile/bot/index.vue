<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRouteParams } from '@vueuse/router';
import CityTable from '@/components/profile/CityTable.vue';
import { usePublicBot } from '@/composables/npc/bot/usePublicBot';
import { usePublicCities } from '@/composables/city/usePublicCities';
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

const id = useRouteParams<Option<BotId>>('id', null);
const { bot, coords } = usePublicBot(id);
const { cities } = usePublicCities(coords);
</script>

<template>
  <div class="game-layout">
    <Card v-if="bot" class="size-full overflow-x-hidden overflow-y-auto">
      <CardHeader>
        <CardTitle>
          <span>{{ bot.id }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0 relative size-full">
        <div class="flex w-full min-w-max flex-col gap-4">
          <Table class="sm:max-w-max md:min-w-50">
            <TableBody>
              <TableRow>
                <TableHead>{{ t('point', 2) }}</TableHead>
                <TableCell>{{ bot.formatScore() }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('rank') }}</TableHead>
                <TableCell>{{ bot.formatRank() }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('type') }}</TableHead>
                <TableCell>{{ t('bot') }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>

          <CityTable :cities class="sm:max-w-max md:min-w-50 pb-8" />
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
