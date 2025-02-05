<script setup lang="ts">
import { useLocale } from '@/locale';
import { Card, Table, TableCell, TableCellLink, TableRow } from '@/components';

const { t } = useLocale();
</script>

<template>
  <Card content-class="p-4 pb-8 pt-0">
    <template #title>
      <span>{{ t('production') }}</span>
    </template>

    <Table>
      <TableRow>
        <TableCellLink to="sawmill">{{ t('wood') }}</TableCellLink>
        <TableCell>{{ t('x-per-hour', [2400]) }}</TableCell>
      </TableRow>
      <TableRow>
        <TableCellLink to="quarry">{{ t('stone') }}</TableCellLink>
        <TableCell>{{ t('x-per-hour', [2400]) }}</TableCell>
      </TableRow>
      <TableRow>
        <TableCellLink to="iron-mine">{{ t('iron') }}</TableCellLink>
        <TableCell>{{ t('x-per-hour', [2400]) }}</TableCell>
      </TableRow>
      <TableRow>
        <TableCellLink to="farm">{{ t('food') }}</TableCellLink>
        <TableCell>{{ t('x-per-hour', [2400]) }}</TableCell>
      </TableRow>
    </Table>
  </Card>
</template>
