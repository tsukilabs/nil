<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { formatInt } from '@/lib/intl';
import { useWarehouse } from '@/composables/infrastructure/useBuilding';
import enUS from '@/locale/en-US/scenes/game/infrastructure/storage.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/storage.json';
import { useStorageStats } from '@/composables/infrastructure/useStorageStats';
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

const warehouse = useWarehouse();
const { level, stats } = useStorageStats(warehouse);
</script>

<template>
  <div class="game-layout">
    <Card v-if="warehouse" class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ `${t('warehouse')} (${t('level-x', [level.current])})` }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0">
        <Table v-if="stats.current">
          <TableHeader>
            <TableRow class="bg-card hover:bg-card">
              <TableHead></TableHead>
              <TableHead>{{ t('capacity') }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow>
              <TableCell class="w-72">
                <span>{{ t('current-capacity') }}</span>
              </TableCell>
              <TableCell>
                <span>{{ formatInt(stats.current.capacity) }}</span>
              </TableCell>
            </TableRow>

            <TableRow v-if="stats.next && !level.isMax">
              <TableCell class="w-72">
                <span>{{ t('capacity-on-level-x', [level.next]) }}</span>
              </TableCell>
              <TableCell>
                <span>{{ formatInt(stats.next.capacity) }}</span>
              </TableCell>
            </TableRow>
          </TableBody>

          <TableFooter>
            <TableRow class="bg-card hover:bg-card">
              <TableCell colspan="2">
                <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                  <div>{{ `${t('maintenance')}:` }}</div>
                  <Food :amount="warehouse.getMaintenance()" />
                </div>
              </TableCell>
            </TableRow>
          </TableFooter>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
