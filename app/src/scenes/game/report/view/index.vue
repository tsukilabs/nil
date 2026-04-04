<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { whenever } from '@vueuse/core';
import SupportReport from './SupportReport.vue';
import { useRouteParams } from '@vueuse/router';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { useReport } from '@/composables/report/useReport';
import { Card, CardContent, CardHeader, CardTitle } from '@ui/card';
import { BattleReportImpl } from '@/core/model/report/battle-report';
import BattleReport from '@/scenes/game/report/view/BattleReport.vue';
import { SupportReportImpl } from '@/core/model/report/support-report';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const reportId = useRouteParams<Option<ReportId>>('id', null);
const { report } = useReport(reportId);

whenever(report, ({ id }) => NIL.report.markRead(id));

async function goToReportForwardScene() {
  if (reportId.value) {
    await go('report-forward', { params: { id: reportId.value } });
  }
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader v-if="report">
        <CardTitle>
          <span>{{ report.getTitle(t) }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full flex flex-col gap-6 overflow-auto px-2 py-0">
        <BattleReport v-if="report && (report instanceof BattleReportImpl)" :report />
        <SupportReport v-else-if="report && (report instanceof SupportReportImpl)" :report />

        <div class="grid grid-cols-1 items-center justify-start gap-4 max-w-max">
          <Button
            :disabled="!reportId"
            role="link"
            tabindex="0"
            @click="goToReportForwardScene"
          >
            <span>{{ t('forward') }}</span>
          </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
