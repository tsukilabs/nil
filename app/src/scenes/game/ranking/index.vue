<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { onKeyDown } from '@vueuse/core';
import Loading from '@/components/Loading.vue';
import { throttle } from 'es-toolkit/function';
import type { Ruler } from '@/types/core/ruler';
import { useRanking } from '@/composables/ranking/useRanking';
import { Card, CardContent, CardHeader, CardTitle } from '@ui/card';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@ui/table';

const { t } = useI18n();

const { ranking, loading, load } = useRanking();

const { id: playerId } = NIL.player.refs();

if (__DESKTOP__) {
  onKeyDown('F5', throttle(load, 1000));
}

function getIdClass(ruler: Ruler) {
  return ruler.kind === 'player' && ruler.id === playerId.value ? 'font-bold' : null;
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
        <Table v-else-if="ranking" class="min-w-max">
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
              <TableCell>
                {{ entry.rank }}
              </TableCell>
              <TableCell :class="getIdClass(entry.ruler)">
                {{ entry.ruler.id }}
              </TableCell>
              <TableCell>
                {{ entry.score }}
              </TableCell>
              <TableCell>
                {{ entry.cities }}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
