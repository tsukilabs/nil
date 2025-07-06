<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useMineProduction } from '@/composables/mine';
import { Card, Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const { village } = NIL.village.refs();

const quarry = computed(() => village.value?.quarry);
const { level, actual, base, stabilityLoss } = useMineProduction(quarry);
</script>

<template>
  <div class="game-layout flex-col">
    <Card v-if="quarry" class="w-full" content-class="px-2">
      <template #title>
        <span>{{ `${$t('quarry')} (${$t('level-x', [level.current])})` }}</span>
      </template>

      <Table>
        <template #header>
          <TableRow class="bg-background hover:bg-background">
            <TableHead />
            <TableHead>{{ $t('current-level') }}</TableHead>
            <TableHead v-if="!level.isMax">{{ $t('next-level') }}</TableHead>
          </TableRow>
        </template>

        <TableRow>
          <TableCell>{{ $t('base-production') }}</TableCell>
          <TableCell>
            <Stone :amount="base.current" />
          </TableCell>
          <TableCell v-if="!level.isMax">
            <Stone :amount="base.next" />
          </TableCell>
        </TableRow>

        <TableRow>
          <TableCell>{{ $t('loss-by-stability') }}</TableCell>
          <TableCell>
            <Stone :amount="stabilityLoss.current" />
          </TableCell>
          <TableCell v-if="!level.isMax">
            <Stone :amount="stabilityLoss.next" />
          </TableCell>
        </TableRow>

        <TableRow>
          <TableCell>{{ $t('current-production') }}</TableCell>
          <TableCell>
            <Stone :amount="actual.current" />
          </TableCell>
          <TableCell v-if="!level.isMax">
            <Stone :amount="actual.next" />
          </TableCell>
        </TableRow>

        <template #footer>
          <TableRow class="bg-background hover:bg-background">
            <TableCell :colspan="level.isMax ? 2 : 3">
              <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                <div>{{ `${$t('maintenance')}:` }}</div>
                <Food :amount="quarry.getMaintenance()" />
              </div>
            </TableCell>
          </TableRow>
        </template>
      </Table>
    </Card>
  </div>
</template>
