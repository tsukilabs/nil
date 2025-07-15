<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useIronMine } from '@/composables/infrastructure/useBuilding';
import enUS from '@/locale/en-US/scenes/game/infrastructure/mine.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/mine.json';
import { useMineProduction } from '@/composables/infrastructure/useMineProduction';
import { Card, Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

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
    <Card v-if="ironMine" class="w-full" content-class="px-2">
      <template #title>
        <span>{{ `${t('iron-mine')} (${t('level-x', [level.current])})` }}</span>
      </template>

      <Table>
        <template #header>
          <TableRow class="bg-background hover:bg-background">
            <TableHead />
            <TableHead>{{ t('current-level') }}</TableHead>
            <TableHead v-if="!level.isMax">{{ t('next-level') }}</TableHead>
          </TableRow>
        </template>

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

        <template #footer>
          <TableRow class="bg-background hover:bg-background">
            <TableCell :colspan="level.isMax ? 2 : 3">
              <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                <div>{{ `${t('maintenance')}:` }}</div>
                <Food :amount="ironMine.getMaintenance()" />
              </div>
            </TableCell>
          </TableRow>
        </template>
      </Table>
    </Card>
  </div>
</template>
