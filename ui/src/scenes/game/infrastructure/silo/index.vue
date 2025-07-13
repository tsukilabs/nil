<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useSilo } from '@/composables/infrastructure/useBuilding';
import enUS from '@/locale/en-US/scenes/game/infrastructure/storage.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/storage.json';
import { useStorageCapacity } from '@/composables/infrastructure/useStorageCapacity';
import { Card, Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const silo = useSilo();
const { level, capacity } = useStorageCapacity(silo);
</script>

<template>
  <div class="game-layout">
    <Card v-if="silo" class="w-full" content-class="px-2">
      <template #title>
        <span>{{ `${t('silo')} (${t('level-x', [level.current])})` }}</span>
      </template>

      <Table>
        <template #header>
          <TableRow class="bg-background hover:bg-background">
            <TableHead></TableHead>
            <TableHead>{{ t('capacity') }}</TableHead>
          </TableRow>
        </template>

        <TableRow>
          <TableCell class="w-72">
            <span>{{ t('current-capacity') }}</span>
          </TableCell>
          <TableCell>
            <span>{{ capacity.current }}</span>
          </TableCell>
        </TableRow>

        <TableRow v-if="!level.isMax">
          <TableCell class="w-72">
            <span>{{ t('capacity-on-level-x', [level.next]) }}</span>
          </TableCell>
          <TableCell>
            <span>{{ capacity.next }}</span>
          </TableCell>
        </TableRow>

        <template #footer>
          <TableRow class="bg-background hover:bg-background">
            <TableCell colspan="2">
              <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                <div>{{ `${t('maintenance')}:` }}</div>
                <Food :amount="silo.getMaintenance()" />
              </div>
            </TableCell>
          </TableRow>
        </template>
      </Table>
    </Card>
  </div>
</template>
