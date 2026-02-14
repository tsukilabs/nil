<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { formatDate } from 'date-fns';
import { computed, ref, watch } from 'vue';
import { useRouteQuery } from '@vueuse/router';
import { hostLocalGameWithSavedata } from '@/core/game';
import { asyncComputed, asyncRef, useMutex } from '@tb-dev/vue';
import { getSavedataFiles, SavedataFile } from '@/core/savedata';
import { useSavedataPlayers } from '@/composables/useSavedataPlayers';
import { goBackIfPreviousIsNotGame, QUERY_LOAD_LOCAL_GAME_PATH } from '@/router';
import {
  Button,
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const path = useRouteQuery<Option<string>>(QUERY_LOAD_LOCAL_GAME_PATH, null);

const playerId = ref<Option<PlayerId>>();
const savedata = ref<Option<SavedataFile>>();

const {
  players,
  loading: isLoadingPlayers,
  load: loadPlayers,
} = useSavedataPlayers(savedata);

const {
  state: savedataDirFiles,
  loading: isLoadingSavedataDirFiles,
  load: loadSavedataFiles,
} = asyncRef([], getSavedataFiles);

const files = asyncComputed([], async () => {
  const _files = [...savedataDirFiles.value];
  if (path.value) {
    _files.unshift(await SavedataFile.read(path.value));
  }

  return _files;
});

const { locked, lock } = useMutex();

const canLoad = computed(() => {
  return (
    Boolean(savedata.value) &&
    Boolean(playerId.value) &&
    players.value.some((player) => player === playerId.value) &&
    !isLoadingSavedataDirFiles.value &&
    !isLoadingPlayers.value &&
    !locked.value
  );
});

const canRemove = computed(() => {
  return (
    files.value.length > 0 &&
    Boolean(savedata.value) &&
    !isLoadingSavedataDirFiles.value &&
    !isLoadingPlayers.value &&
    !locked.value
  );
});

watch(files, (value) => {
  savedata.value ??= value.at(0);
});

watch(savedata, async (value) => {
  await loadPlayers();
  if (value && players.value.every((player) => player !== playerId.value)) {
    playerId.value = players.value.at(0);
  }
});

async function load() {
  await lock(async () => {
    if (savedata.value && playerId.value) {
      await hostLocalGameWithSavedata({
        path: savedata.value.path,
        playerOptions: {
          id: playerId.value,
        },
      });
    }
  });
}

async function remove() {
  await lock(async () => {
    if (savedata.value) {
      await savedata.value.remove();
      savedata.value = null;
      await loadSavedataFiles();
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

      <CardContent class="h-full overflow-hidden px-2!">
        <div class="w-full">
          <Select v-model="playerId" :placeholder="t('player')">
            <SelectTrigger class="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="player of players" :key="player" :value="player">
                {{ player }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="h-full overflow-x-hidden overflow-y-auto md:px-2">
          <div class="flex flex-col items-start gap-1 p-0">
            <Button
              v-for="file of files"
              :key="file.name"
              :variant="file.path === savedata?.path ? 'secondary' : 'ghost'"
              class="h-16 w-full flex flex-col items-start gap-1"
              @click="() => void (savedata = file)"
            >
              <span class="ellipsis">{{ file.info.worldName }}</span>
              <div class="flex gap-4 text-muted-foreground text-xs">
                <span>{{ t('round-x', [file.info.round]) }}</span>
                <span>{{ formatDate(file.date, 'dd/MM/yyyy HH:mm:ss') }}</span>
              </div>
            </Button>
          </div>
        </div>
      </CardContent>

      <CardFooter class="grid grid-cols-3">
        <Button :disabled="!canLoad" @click="load">
          {{ t('load') }}
        </Button>
        <Button variant="destructive" :disabled="!canRemove" @click="remove">
          {{ t('delete') }}
        </Button>
        <Button variant="secondary" :disabled="locked" @click="goBackIfPreviousIsNotGame">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
