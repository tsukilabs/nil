<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { formatInt, formatPercent } from '@/lib/intl';
import { useWall } from '@/composables/infrastructure/useBuilding';
import enUS from '@/locale/en-US/scenes/game/infrastructure/wall.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/wall.json';
import { useWallStats } from '@/composables/infrastructure/useWallStats';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const wall = useWall();
const { level, stats } = useWallStats(wall);
</script>

<template>
  <div class="game-layout">
    <Card v-if="wall" class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ `${t('wall')} (${t('level-x', [level.current])})` }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0">
        <Table v-if="stats.current">
          <TableHeader>
            <TableRow class="bg-card hover:bg-card">
              <TableHead />
              <TableHead>{{ t('current-level') }}</TableHead>
              <TableHead v-if="stats.next && !level.isMax">
                {{ t('next-level') }}
              </TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow>
              <TableCell>{{ t('basic-defense') }}</TableCell>
              <TableCell>
                <span>{{ formatInt(stats.current.defense) }}</span>
              </TableCell>
              <TableCell v-if="stats.next && !level.isMax">
                <span>{{ formatInt(stats.next.defense) }}</span>
              </TableCell>
            </TableRow>

            <TableRow>
              <TableCell>{{ t('defensive-bonus') }}</TableCell>
              <TableCell>
                <span>{{ formatPercent(stats.current.defensePercent / 100) }}</span>
              </TableCell>
              <TableCell v-if="stats.next && !level.isMax">
                <span>{{ formatPercent(stats.next.defensePercent / 100) }}</span>
              </TableCell>
            </TableRow>
          </TableBody>

          <TableFooter>
            <TableRow class="bg-card hover:bg-card">
              <TableCell colspan="3">
                <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                  <div>{{ `${t('maintenance')}:` }}</div>
                  <Food :amount="wall.getMaintenance()" />
                </div>
              </TableCell>
            </TableRow>
          </TableFooter>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
