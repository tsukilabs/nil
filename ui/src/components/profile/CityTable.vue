<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { formatInt } from '@/lib/intl';
import type { PublicCityImpl } from '@/core/model/city/public-city';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

defineProps<{
  cities: readonly PublicCityImpl[];
}>();

const { t } = useI18n();
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow class="hover:bg-card">
        <TableHead>{{ t('city', 2) }}</TableHead>
        <TableHead>{{ t('coordinate', 2) }}</TableHead>
        <TableHead>{{ t('point', 2) }}</TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <TableRow
        v-for="city of cities"
        :key="city.index"
        role="link"
        tabindex="0"
        class="cursor-pointer"
        @click="() => city.goToProfile()"
        @keydown.enter="() => city.goToProfile()"
      >
        <TableCell>{{ city.name }}</TableCell>
        <TableCell>{{ city.coord.format() }}</TableCell>
        <TableCell>{{ formatInt(city.score) }}</TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
