<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { throttle } from 'es-toolkit';
import { onKeyDown } from '@vueuse/core';
import Loading from '@/components/Loading.vue';
import { useRanking } from '@/composables/ranking/useRanking';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const { ranking, loading, load } = useRanking();

if (__DESKTOP__) {
  onKeyDown('F5', throttle(load, 1000));
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t('ranking') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="relative size-full overflow-auto px-2 py-0">
        <Loading v-if="loading" />
        <Table v-else-if="ranking">
          <TableHeader>
            <TableRow class="hover:bg-card">
              <TableHead>{{ t('rank') }}</TableHead>
              <TableHead>{{ t('name') }}</TableHead>
              <TableHead>{{ t('point', 2) }}</TableHead>
              <TableHead>{{ t('city', 2) }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow
              v-for="entry of ranking"
              :key="entry.uniqueId"
              role="link"
              tabindex="0"
              class="cursor-pointer"
              @click="() => entry.goToProfile()"
              @keydown.enter="() => entry.goToProfile()"
            >
              <TableCell>{{ entry.formatRank() }}</TableCell>
              <TableCell>{{ entry.ruler.id }}</TableCell>
              <TableCell>{{ entry.formatScore() }}</TableCell>
              <TableCell>{{ entry.formatCities() }}</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
