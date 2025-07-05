<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { Card, Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const { village } = NIL.village.refs();

const ironMine = computed(() => village.value?.ironMine);
const isMaxLevel = computed(() => ironMine.value?.isMaxLevel());

const level = computed(() => {
  const current = ironMine.value?.level ?? 0;
  return { current, next: current + 1 };
});

const base = computed(() => {
  return {
    current: ironMine.value?.getProduction() ?? 0,
    next: ironMine.value?.getProductionBy(level.value.next) ?? 0,
  };
});

const stabilityLoss = computed(() => {
  const stability = village.value?.stability ?? 1;
  return {
    current: base.value.current - base.value.current * stability,
    next: base.value.next - base.value.next * stability,
  };
});

const actual = computed(() => {
  const current = Math.ceil(base.value.current - stabilityLoss.value.current);
  const next = Math.ceil(base.value.next - stabilityLoss.value.next);
  return { current: Math.max(current, 0), next: Math.max(next, 0) };
});
</script>

<template>
  <div class="game-layout flex-col">
    <Card v-if="ironMine" class="w-full" content-class="px-2">
      <template #title>
        <span>{{ `${$t('iron-mine')} (${$t('level-x', [level.current])})` }}</span>
      </template>

      <Table>
        <template #header>
          <TableRow class="bg-background hover:bg-background">
            <TableHead />
            <TableHead>{{ $t('current-level') }}</TableHead>
            <TableHead v-if="!isMaxLevel">{{ $t('next-level') }}</TableHead>
          </TableRow>
        </template>

        <TableRow>
          <TableCell>{{ $t('base-production') }}</TableCell>
          <TableCell>
            <Iron :amount="base.current" />
          </TableCell>
          <TableCell v-if="!isMaxLevel">
            <Iron :amount="base.next" />
          </TableCell>
        </TableRow>

        <TableRow>
          <TableCell>{{ $t('loss-by-stability') }}</TableCell>
          <TableCell>
            <Iron :amount="stabilityLoss.current" />
          </TableCell>
          <TableCell v-if="!isMaxLevel">
            <Iron :amount="stabilityLoss.next" />
          </TableCell>
        </TableRow>

        <TableRow>
          <TableCell>{{ $t('current-production') }}</TableCell>
          <TableCell>
            <Iron :amount="actual.current" />
          </TableCell>
          <TableCell v-if="!isMaxLevel">
            <Iron :amount="actual.next" />
          </TableCell>
        </TableRow>

        <template #footer>
          <TableRow class="bg-background hover:bg-background">
            <TableCell />
            <TableCell />
            <TableCell>
              <div class="flex w-full items-center justify-end gap-2 px-2 pt-4">
                <div>{{ `${$t('maintenance')}:` }}</div>
                <Food :amount="ironMine.getMaintenance()" />
              </div>
            </TableCell>
          </TableRow>
        </template>
      </Table>
    </Card>
  </div>
</template>
