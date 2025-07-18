<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useIronMine } from '@/composables/infrastructure/useBuilding';
import enUS from '@/locale/en-US/scenes/game/infrastructure/mine.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/mine.json';
import { useMineProduction } from '@/composables/infrastructure/useMineProduction';
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

const ironMine = useIronMine();
const { level, actual, base, stabilityLoss } = useMineProduction(ironMine);
</script>

<template>
  <div class="game-layout flex-col">
    <Card v-if="ironMine" class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ `${t('iron-mine')} (${t('level-x', [level.current])})` }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0">
        <Table>
          <TableHeader>
            <TableRow class="hover:bg-card">
              <TableHead />
              <TableHead>{{ t('current-level') }}</TableHead>
              <TableHead v-if="!level.isMax">{{ t('next-level') }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow>
              <TableCell>{{ t('base-production') }}</TableCell>
              <TableCell>
                <Iron :amount="base.current" />
              </TableCell>
              <TableCell v-if="!level.isMax">
                <Iron :amount="base.next" />
              </TableCell>
            </TableRow>

            <TableRow>
              <TableCell>{{ t('loss-by-stability') }}</TableCell>
              <TableCell>
                <Iron :amount="stabilityLoss.current" />
              </TableCell>
              <TableCell v-if="!level.isMax">
                <Iron :amount="stabilityLoss.next" />
              </TableCell>
            </TableRow>

            <TableRow>
              <TableCell>{{ t('current-production') }}</TableCell>
              <TableCell>
                <Iron :amount="actual.current" />
              </TableCell>
              <TableCell v-if="!level.isMax">
                <Iron :amount="actual.next" />
              </TableCell>
            </TableRow>
          </TableBody>

          <TableFooter>
            <TableRow class="bg-card hover:bg-card">
              <TableCell :colspan="level.isMax ? 2 : 3">
                <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                  <div>{{ `${t('maintenance')}:` }}</div>
                  <Food :amount="ironMine.getMaintenance()" />
                </div>
              </TableCell>
            </TableRow>
          </TableFooter>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
