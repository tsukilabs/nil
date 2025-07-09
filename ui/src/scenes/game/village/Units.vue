<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Card, Table, TableCell, TableRow } from '@tb-dev/vue-components';

const { t } = useI18n();
</script>

<template>
  <Card content-class="px-2">
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
