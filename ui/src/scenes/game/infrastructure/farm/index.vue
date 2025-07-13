<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useFarm } from '@/composables/infrastructure/useBuilding';
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

const farm = useFarm();
const { level, actual, base, stabilityLoss } = useMineProduction(farm);
</script>

<template>
  <div class="game-layout flex-col">
    <Card v-if="farm" class="w-full" content-class="px-2">
      <template #title>
        <span>{{ `${t('farm')} (${t('level-x', [level.current])})` }}</span>
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
            <Food :amount="base.current" />
          </TableCell>
          <TableCell v-if="!level.isMax">
            <Food :amount="base.next" />
          </TableCell>
        </TableRow>

        <TableRow>
          <TableCell>{{ t('loss-by-stability') }}</TableCell>
          <TableCell>
            <Food :amount="stabilityLoss.current" />
          </TableCell>
          <TableCell v-if="!level.isMax">
            <Food :amount="stabilityLoss.next" />
          </TableCell>
        </TableRow>

        <TableRow>
          <TableCell>{{ t('current-production') }}</TableCell>
          <TableCell>
            <Food :amount="actual.current" />
          </TableCell>
          <TableCell v-if="!level.isMax">
            <Food :amount="actual.next" />
          </TableCell>
        </TableRow>

        <template #footer>
          <TableRow class="bg-background hover:bg-background">
            <TableCell :colspan="level.isMax ? 2 : 3">
              <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                <div>{{ `${t('maintenance')}:` }}</div>
                <Food :amount="farm.getMaintenance()" />
              </div>
            </TableCell>
          </TableRow>
        </template>
      </Table>
    </Card>
  </div>
</template>
