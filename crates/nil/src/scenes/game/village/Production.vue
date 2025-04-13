<script setup lang="ts">
import { useLocale } from '@/locale';
import { Card, Table, TableCell, TableLink, TableRow } from '@tb-dev/vue';

const { t } = useLocale();
</script>

<template>
  <Card content-class="p-4 pt-0 pb-8">
    <template #title>
      <span>{{ t('production') }}</span>
    </template>

    <Table>
      <TableRow>
        <TableLink to="sawmill">{{ t('wood') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [2400]) }}</TableCell>
      </TableRow>

      <TableRow>
        <TableLink to="quarry">{{ t('stone') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [2400]) }}</TableCell>
      </TableRow>

      <TableRow>
        <TableLink to="iron-mine">{{ t('iron') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [2400]) }}</TableCell>
      </TableRow>

      <TableRow>
        <TableLink to="farm">{{ t('food') }}</TableLink>
        <TableCell>{{ t('x-per-turn', [2400]) }}</TableCell>
      </TableRow>
    </Table>
  </Card>
</template>
