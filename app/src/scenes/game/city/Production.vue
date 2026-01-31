<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const { production } = NIL.city.refs();
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
        <span>{{ t('production') }}</span>
      </CardTitle>
    </CardHeader>

    <CardContent class="px-2 py-0">
      <Table>
        <TableBody>
          <TableRow
            v-if="production?.wood"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click="() => go('sawmill')"
          >
            <TableCell>{{ t('wood') }}</TableCell>
            <TableCell>{{ t('x-per-turn', [production.wood]) }}</TableCell>
          </TableRow>

          <TableRow
            v-if="production?.stone"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click="() => go('quarry')"
          >
            <TableCell>{{ t('stone') }}</TableCell>
            <TableCell>{{ t('x-per-turn', [production.stone]) }}</TableCell>
          </TableRow>

          <TableRow
            v-if="production?.iron"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click="() => go('iron-mine')"
          >
            <TableCell>{{ t('iron') }}</TableCell>
            <TableCell>{{ t('x-per-turn', [production.iron]) }}</TableCell>
          </TableRow>

          <TableRow
            v-if="production?.food"
            role="link"
            tabindex="0"
            class="cursor-pointer"
            @click="() => go('farm')"
          >
            <TableCell>{{ t('food') }}</TableCell>
            <TableCell>{{ t('x-per-turn', [production.food]) }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
