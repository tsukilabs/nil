<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useSawmill } from '@/composables/infrastructure/useBuilding';
import enUS from '@/locale/en-US/scenes/game/infrastructure/mine.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/mine.json';
import { useMineStats } from '@/composables/infrastructure/useMineStats';
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

const sawmill = useSawmill();
const { level, stats, actual, base, stabilityLoss } = useMineStats(sawmill);
</script>

<template>
  <div class="game-layout">
    <Card v-if="sawmill" class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ `${t('sawmill')} (${t('level-x', [level.current])})` }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0">
        <Table>
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
              <TableCell>{{ t('base-production') }}</TableCell>
              <TableCell>
                <Wood :amount="base.current" />
              </TableCell>
              <TableCell v-if="stats.next && !level.isMax">
                <Wood :amount="base.next" />
              </TableCell>
            </TableRow>

            <TableRow>
              <TableCell>{{ t('loss-by-stability') }}</TableCell>
              <TableCell>
                <Wood :amount="stabilityLoss.current" />
              </TableCell>
              <TableCell v-if="stats.next && !level.isMax">
                <Wood :amount="stabilityLoss.next" />
              </TableCell>
            </TableRow>

            <TableRow>
              <TableCell>{{ t('current-production') }}</TableCell>
              <TableCell>
                <Wood :amount="actual.current" />
              </TableCell>
              <TableCell v-if="stats.next && !level.isMax">
                <Wood :amount="actual.next" />
              </TableCell>
            </TableRow>
          </TableBody>

          <TableFooter>
            <TableRow class="bg-card hover:bg-card">
              <TableCell :colspan="!stats.next || level.isMax ? 2 : 3">
                <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                  <div>{{ `${t('maintenance')}:` }}</div>
                  <Food :amount="sawmill.getMaintenance()" />
                </div>
              </TableCell>
            </TableRow>
          </TableFooter>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
