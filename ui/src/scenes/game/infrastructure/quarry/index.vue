<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useQuarry } from '@/composables/infrastructure/useBuilding';
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

const quarry = useQuarry();
const { level, actual, base, stabilityLoss } = useMineProduction(quarry);
</script>

<template>
  <div class="game-layout flex-col">
    <Card v-if="quarry" class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ `${t('quarry')} (${t('level-x', [level.current])})` }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0">
        <Table>
          <TableHeader>
            <TableRow class="bg-card hover:bg-card">
              <TableHead />
              <TableHead>{{ t('current-level') }}</TableHead>
              <TableHead v-if="!level.isMax">{{ t('next-level') }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow>
              <TableCell>{{ t('base-production') }}</TableCell>
              <TableCell>
                <Stone :amount="base.current" />
              </TableCell>
              <TableCell v-if="!level.isMax">
                <Stone :amount="base.next" />
              </TableCell>
            </TableRow>

            <TableRow>
              <TableCell>{{ t('loss-by-stability') }}</TableCell>
              <TableCell>
                <Stone :amount="stabilityLoss.current" />
              </TableCell>
              <TableCell v-if="!level.isMax">
                <Stone :amount="stabilityLoss.next" />
              </TableCell>
            </TableRow>

            <TableRow>
              <TableCell>{{ t('current-production') }}</TableCell>
              <TableCell>
                <Stone :amount="actual.current" />
              </TableCell>
              <TableCell v-if="!level.isMax">
                <Stone :amount="actual.next" />
              </TableCell>
            </TableRow>
          </TableBody>

          <TableFooter>
            <TableRow class="bg-card hover:bg-card">
              <TableCell :colspan="level.isMax ? 2 : 3">
                <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                  <div>{{ `${t('maintenance')}:` }}</div>
                  <Food :amount="quarry.getMaintenance()" />
                </div>
              </TableCell>
            </TableRow>
          </TableFooter>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
