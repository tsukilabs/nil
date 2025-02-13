<script setup lang="ts">
import { useLocale } from '@/locale';
import { Card, Table, TableCell, TableRow } from '@/components';

const { t } = useLocale();
</script>

<template>
  <Card content-class="p-4 pb-8 pt-0">
    <template #title>
      <span>{{ t('unit', 2) }}</span>
    </template>

    <Table>
      <TableRow>
        <TableCell>{{ `8000 ${t('pikeman', 8000)}` }}</TableCell>
      </TableRow>

      <TableRow>
        <TableCell>{{ `8000 ${t('archer', 8000)}` }}</TableCell>
      </TableRow>

      <TableRow>
        <TableCell>{{ `1000 ${t('heavy-cavalry', 1000)}` }}</TableCell>
      </TableRow>
    </Table>
  </Card>
</template>
