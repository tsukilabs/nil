<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
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

const { production } = NIL.village.refs();
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
          <TableRow v-if="production?.wood">
            <TableCell>
              <RouterLink :to="{ name: 'sawmill' satisfies InfrastructureScene }">
                {{ t('wood') }}
              </RouterLink>
            </TableCell>
            <TableCell>{{ t('x-per-turn', [production.wood]) }}</TableCell>
          </TableRow>

          <TableRow v-if="production?.stone">
            <TableCell>
              <RouterLink :to="{ name: 'quarry' satisfies InfrastructureScene }">
                {{ t('stone') }}
              </RouterLink>
            </TableCell>
            <TableCell>{{ t('x-per-turn', [production.stone]) }}</TableCell>
          </TableRow>

          <TableRow v-if="production?.iron">
            <TableCell>
              <RouterLink :to="{ name: 'iron-mine' satisfies InfrastructureScene }">
                {{ t('iron') }}
              </RouterLink>
            </TableCell>
            <TableCell>{{ t('x-per-turn', [production.iron]) }}</TableCell>
          </TableRow>

          <TableRow v-if="production?.food">
            <TableCell>
              <RouterLink :to="{ name: 'farm' satisfies InfrastructureScene }">
                {{ t('food') }}
              </RouterLink>
            </TableCell>
            <TableCell>{{ t('x-per-turn', [production.food]) }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
