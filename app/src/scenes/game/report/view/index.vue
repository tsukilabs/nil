<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { whenever } from '@vueuse/core';
import { useRouteParams } from '@vueuse/router';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { useReport } from '@/composables/report/useReport';
import { BattleReportImpl } from '@/core/model/report/battle-report';
import BattleReport from '@/scenes/game/report/view/BattleReport.vue';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const reportId = useRouteParams<Option<ReportId>>('id', null);
const { report } = useReport(reportId);

whenever(report, ({ id }) => NIL.report.markRead(id));
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader v-if="report">
        <CardTitle>
          <span>{{ report.getTitle(t) }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="relative size-full overflow-auto px-2 py-0">
        <BattleReport v-if="report && (report instanceof BattleReportImpl)" :report />
      </CardContent>
    </Card>
  </div>
</template>
