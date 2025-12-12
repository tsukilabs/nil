<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { formatDate } from 'date-fns';
import { computed, onMounted, ref } from 'vue';
import { isPlayerOptions } from '@/lib/schema';
import { hostWithSavedata } from '@/core/game';
import { useRoute, useRouter } from 'vue-router';
import type { WritablePartial } from '@tb-dev/utils';
import { asyncRef, localRef, useMutex } from '@tb-dev/vue';
import { getSavedataFiles, type SavedataFile } from '@/core/savedata';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n();

const router = useRouter();
const route = useRoute();

const player = localRef<WritablePartial<PlayerOptions>>('load-game:player', {
  id: null,
});

const savedata = ref<Option<SavedataFile>>();
const { state: files, isLoading, execute } = asyncRef([], getSavedataFiles);

const { locked, lock } = useMutex();
const isValidPlayer = computed(() => isPlayerOptions(player.value));

const canLoad = computed(() => {
  return (
    Boolean(savedata.value) &&
    isValidPlayer.value &&
    !isLoading.value &&
    !locked.value
  );
});

const canRemove = computed(() => {
  return (
    files.value.length > 0 &&
    Boolean(savedata.value) &&
    !isLoading.value &&
    !locked.value
  );
});

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

async function remove() {
  await lock(async () => {
    if (savedata.value) {
      await savedata.value.remove();
      savedata.value = null;
      await execute();
    }
  });
}
</script>

<template>
  <div class="card-layout">
    <Card class="sm:max-w-4/5 h-full max-h-[95%] sm:max-h-3/5">
      <CardHeader>
        <CardTitle>{{ t('load-game') }}</CardTitle>
      </CardHeader>

      <CardContent class="h-full overflow-x-hidden overflow-y-auto">
        <div class="flex flex-col items-start gap-1 p-0">
          <Button
            v-for="file of files"
            :key="file.name"
            :variant="file.path === savedata?.path ? 'secondary' : 'ghost'"
            class="h-16 w-full flex flex-col items-start gap-1"
            @click="() => (savedata = file)"
          >
            <span class="ellipsis">{{ file.info.worldName }}</span>
            <div class="flex gap-4 text-muted-foreground text-xs">
              <span>{{ t('round-x', [file.info.round]) }}</span>
              <span>{{ formatDate(file.date, 'dd/MM/yyyy HH:mm:ss') }}</span>
            </div>
          </Button>
        </div>
      </CardContent>

      <CardFooter class="grid grid-cols-3">
        <Button :disabled="!canLoad" @click="load">
          {{ t('load') }}
        </Button>
        <Button variant="destructive" :disabled="!canRemove" @click="remove">
          {{ t('delete') }}
        </Button>
        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
