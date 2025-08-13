<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, onMounted, ref } from 'vue';
import { isPlayerOptions } from '@/lib/schema';
import { hostWithSavedata } from '@/core/game';
import { useRoute, useRouter } from 'vue-router';
import type { WritablePartial } from '@tb-dev/utils';
import enUS from '@/locale/en-US/scenes/load-game.json';
import ptBR from '@/locale/pt-BR/scenes/load-game.json';
import { asyncRef, localRef, useMutex } from '@tb-dev/vue';
import { compareDesc as compareDateDesc, formatDate } from 'date-fns';
import { getSavedataFiles, type SavedataFile } from '@/core/savedata';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const route = useRoute();

const player = localRef<WritablePartial<PlayerOptions>>('load-game:player', {
  id: null,
});

const savedata = ref<Option<SavedataFile>>();
const { state: files } = asyncRef([], async () => {
  const sfiles = await getSavedataFiles();
  return sfiles.toSorted((a, b) => compareDateDesc(a.date, b.date));
});

const { locked, lock } = useMutex();
const isValidPlayer = computed(() => isPlayerOptions(player.value));
const canLoad = computed(() => Boolean(savedata.value) && isValidPlayer.value);

onMounted(() => {
  if (typeof route.query.playerId === 'string') {
    player.value.id = route.query.playerId;
  }
});

async function load() {
  await lock(async () => {
    if (savedata.value && isPlayerOptions(player.value)) {
      await hostWithSavedata(savedata.value.path, player.value);
    }
  });
}
</script>

<template>
  <div class="card-layout">
    <Card class="sm:max-w-4/5 max-h-3/5">
      <CardHeader>
        <CardTitle>{{ t('load-game') }}</CardTitle>
      </CardHeader>

      <CardContent class="overflow-x-hidden overflow-y-auto">
        <div class="flex flex-col items-start gap-1 p-0">
          <Button
            v-for="file of files"
            :key="file.path"
            :variant="file.path === savedata?.path ? 'secondary' : 'ghost'"
            class="h-16 w-full flex flex-col items-start gap-1"
            @click="() => (savedata = file)"
          >
            <span class="ellipsis">{{ file.name }}</span>
            <span class="text-muted-foreground text-xs">
              {{ formatDate(file.date, 'dd/MM/yyyy HH:mm:ss') }}
            </span>
          </Button>
        </div>
      </CardContent>

      <CardFooter class="grid grid-cols-2">
        <Button :disabled="locked || !canLoad" @click="load">
          {{ t('load') }}
        </Button>
        <Button variant="secondary" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
