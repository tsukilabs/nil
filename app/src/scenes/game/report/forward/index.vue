<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { computed, ref } from 'vue';
import { Checkbox } from '@ui/checkbox';
import { useRouteParams } from '@vueuse/router';
import { useRoute, useRouter } from 'vue-router';
import { forwardReport } from '@/commands/report';
import { toBooleanCheckboxValue } from '@ui/utils';
import { useBreakpoints, useMutex } from '@tb-dev/vue';
import { useReport } from '@/composables/report/useReport';
import { usePlayerIds } from '@/composables/player/usePlayerIds';
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

const search = ref('');
const { player } = NIL.player.refs();

const { playerIds } = usePlayerIds({
  exclude: reportPlayers,
  filter: (id) => id !== player.value?.id,
  search,
});

const selected = ref<PlayerId[]>([]);

const { locked, lock } = useMutex();

async function forward() {
  await lock(async () => {
    const ids = selected.value.filter((id) => {
      return !reportPlayers.value.includes(id);
    });

    if (reportId.value && ids.length > 0) {
      await Promise.all(
        ids.map((playerId) => {
          return forwardReport(reportId.value!, playerId);
        }),
      );

      if (route.name === ('report-forward' satisfies ReportScene)) {
        router.back();
      }
    }
  });
}

function onChecked(playerId: PlayerId, checked: boolean) {
  if (checked) {
    selected.value.push(playerId);
  }
  else {
    selected.value = selected.value.filter((id) => id !== playerId);
  }
}

function clear() {
  search.value = '';
  selected.value = [];
}
</script>

<template>
  <div :class="md ? 'card-layout' : 'game-layout'">
    <Card class="max-md:size-full overflow-hidden md:max-h-9/10">
      <CardHeader>
        <CardTitle>
          <span>{{ t('forward') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full overflow-hidden px-2 py-0">
        <div class="w-full">
          <InputPlayerName v-model="search" />
        </div>
        <div class="h-full overflow-x-hidden overflow-y-auto md:px-2">
          <Label v-for="playerId of playerIds" :key="playerId">
            <Checkbox
              :model-value="selected.includes(playerId)"
              :disabled="locked"
              @update:model-value="(checked) => onChecked(playerId, toBooleanCheckboxValue(checked))"
            />
            <span>{{ playerId }}</span>
          </Label>
        </div>
      </CardContent>

      <CardFooter class="w-full grid grid-cols-3 gap-2">
        <Button :disabled="locked || selected.length === 0" @click="forward">
          <span>{{ t('forward') }}</span>
        </Button>

        <Button variant="secondary" :disabled="locked" @click="clear">
          <span>{{ t('clear') }}</span>
        </Button>

        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
