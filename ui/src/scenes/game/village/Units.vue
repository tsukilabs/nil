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
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
        <span>{{ t('unit', 2) }}</span>
      </CardTitle>
    </CardHeader>

    <CardContent class="px-2 py-0">
      <Table>
        <TableBody>
          <TableRow>
            <TableCell>{{ `8000 ${t('pikeman', 8000)}` }}</TableCell>
          </TableRow>
          <TableRow>
            <TableCell>{{ `8000 ${t('archer', 8000)}` }}</TableCell>
          </TableRow>
          <TableRow>
            <TableCell>{{ `1000 ${t('heavy-cavalry', 1000)}` }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
