<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Card, Table, TableCell, TableLink, TableRow } from '@tb-dev/vue-components';

const { t } = useI18n();

const { production } = NIL.village.refs();
</script>

<template>
  <Card content-class="px-2">
    <template #title>
      <span>{{ t('production') }}</span>
    </template>

    <Table>
      <TableRow v-if="production?.wood">
        <TableLink to="sawmill">{{ t('wood') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [production.wood]) }}</TableCell>
      </TableRow>

      <TableRow v-if="production?.stone">
        <TableLink to="quarry">{{ t('stone') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [production.stone]) }}</TableCell>
      </TableRow>

      <TableRow v-if="production?.iron">
        <TableLink to="iron-mine">{{ t('iron') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [production.iron]) }}</TableCell>
      </TableRow>

      <TableRow v-if="production?.food">
        <TableLink to="farm">{{ t('food') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [production.food]) }}</TableCell>
      </TableRow>
    </Table>
  </Card>
</template>
