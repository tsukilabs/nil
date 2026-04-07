<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { useRoute } from 'vue-router';
import * as commands from '@/commands';
import { whenever } from '@vueuse/core';
import SupportReport from './SupportReport.vue';
import { useRouteParams } from '@vueuse/router';
import type { ReportId } from '@/types/core/report';
import type { ReportScene } from '@/types/scene/game';
import { useBreakpoints, useMutex } from '@tb-dev/vue';
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

const route = useRoute();

const reportId = useRouteParams<Option<ReportId>>('id', null);
const { report } = useReport(reportId);

const { sm } = useBreakpoints();

const { locked, lock } = useMutex();

whenever(report, ({ id }) => NIL.report.markRead(id));

async function goToReportForwardScene() {
  if (report.value) {
    await go('report-forward', { params: { id: report.value.id } });
  }
}

async function remove() {
  await lock(async () => {
    if (report.value) {
      const id = report.value.id;
      await commands.removeReport(id);
      await NIL.report.update();

      if (
        reportId.value === id &&
        route.name === ('report-view' satisfies ReportScene)
      ) {
        await go('report');
      }
    }
  });
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

        <div v-if="report" class="grid grid-cols-2 items-center justify-start gap-4 max-w-max">
          <Button
            variant="default"
            :size="sm ? 'default' : 'xs'"
            :disabled="locked"
            role="link"
            tabindex="0"
            @click="goToReportForwardScene"
          >
            <span>{{ t('forward') }}</span>
          </Button>
          <Button
            variant="destructive"
            :size="sm ? 'default' : 'xs'"
            :disabled="locked"
            @click="remove"
          >
            <span>{{ t('remove') }}</span>
          </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
