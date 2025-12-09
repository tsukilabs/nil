<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { useReports } from '@/composables/report/useReports';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { reports: reportIds } = NIL.report.refs();
const { reports } = useReports(reportIds);
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t('report', 2) }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="relative size-full overflow-auto px-2 py-0">
        <Table v-if="reports.length > 0">
          <TableHeader>
            <TableRow class="hover:bg-card">
              <TableHead>{{ t('title') }}</TableHead>
              <TableHead>{{ t('date') }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow
              v-for="report of reports"
              :key="report.id"
              role="link"
              tabindex="0"
              class="cursor-pointer"
              @click="() => report.goToView()"
            >
              <TableCell>{{ report.getTitle() }}</TableCell>
              <TableCell>{{ report.formatDate() }}</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
