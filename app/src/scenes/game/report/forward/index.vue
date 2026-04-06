<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { computed, ref } from 'vue';
import { useRouteParams } from '@vueuse/router';
import { useRoute, useRouter } from 'vue-router';
import { forwardReport } from '@/commands/report';
import type { PlayerId } from '@/types/core/player';
import type { ReportId } from '@/types/core/report';
import type { ReportScene } from '@/types/scene/game';
import { useBreakpoints, useMutex } from '@tb-dev/vue';
import { useReport } from '@/composables/report/useReport';
import InputPlayerName from '@/components/form/InputPlayerName.vue';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@ui/card';

const { t } = useI18n();

const route = useRoute();
const router = useRouter();

const reportId = useRouteParams<Option<ReportId>>('id', null);
const { report } = useReport(reportId);

const reportPlayers = computed(() => {
  return report.value ? report.value.getPlayerIds() : [];
});

const { md } = useBreakpoints();

const player = ref<Option<PlayerId>>();

const { locked, lock } = useMutex();

const canForward = computed(() => {
  return Boolean(
    report.value &&
      player.value &&
      !reportPlayers.value.includes(player.value),
  );
});

async function forward() {
  await lock(async () => {
    if (canForward.value && reportId.value && player.value) {
      await forwardReport(reportId.value, player.value);

      if (route.name === ('report-forward' satisfies ReportScene)) {
        router.back();
      }
    }
  });
}

function clear() {
  player.value = null;
}
</script>

<template>
  <div :class="md ? 'card-layout' : 'game-layout'">
    <Card class="max-md:size-full md:max-h-[95%] overflow-hidden">
      <CardHeader>
        <CardTitle>
          <span>{{ t('forward') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full overflow-hidden px-2 py-0">
        <InputPlayerName v-model="player" />
      </CardContent>

      <CardFooter class="w-full grid grid-cols-3 gap-2">
        <Button :disabled="locked || !canForward" @click="forward">
          <span>{{ t('forward') }}</span>
        </Button>

        <Button variant="secondary" :disabled="locked || !player" @click="clear">
          <span>{{ t('clear') }}</span>
        </Button>

        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
