<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { PublicVillageImpl } from '@/core/model/village/public-village';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

defineProps<{
  villages: readonly PublicVillageImpl[];
}>();

const { t } = useI18n();
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow class="hover:bg-card">
        <TableHead>{{ t('village', 2) }}</TableHead>
        <TableHead>{{ t('coordinate', 2) }}</TableHead>
        <TableHead>{{ t('point', 2) }}</TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <TableRow
        v-for="village of villages"
        :key="village.index"
        role="link"
        tabindex="0"
        class="cursor-pointer"
        @click="() => village.goToProfile()"
        @keydown.enter="() => village.goToProfile()"
      >
        <TableCell>{{ village.name }}</TableCell>
        <TableCell>{{ village.coord.format() }}</TableCell>
        <TableCell>???</TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
