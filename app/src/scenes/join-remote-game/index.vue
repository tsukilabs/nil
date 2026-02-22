<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Box from './Box.vue';
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { useMutex } from '@tb-dev/vue';
import { useRouter } from 'vue-router';
import { formatToday } from '@/lib/date';
import { joinRemoteGame } from '@/core/game';
import Loading from '@/components/Loading.vue';
import { isValidPassword } from '@/lib/schema';
import { useRouteQuery } from '@vueuse/router';
import { useSettings } from '@/stores/settings';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { QUERY_JOIN_REMOTE_GAME_WORLD_ID } from '@/router';
import ButtonSpinner from '@/components/button/ButtonSpinner.vue';
import { useRemoteWorld } from '@/composables/world/useRemoteWorld';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle, Input } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const worldId = useRouteQuery<Option<WorldId>>(QUERY_JOIN_REMOTE_GAME_WORLD_ID, null);

const settings = useSettings();

const { remoteWorld, loading } = useRemoteWorld(worldId);

const worldPassword = ref<Option<string>>();

const { locked, lock } = useMutex();
const canJoin = computed(() => {
  return (
    Boolean(remoteWorld.value) &&
    Boolean(settings.auth.token) &&
    (!remoteWorld.value?.hasPassword || isValidPassword(worldPassword.value))
  );
});

async function join() {
  await lock(async () => {
    if (remoteWorld.value && settings.auth.token) {
      await joinRemoteGame({
        worldId: remoteWorld.value.id,
        worldPassword: worldPassword.value,
        authorizationToken: settings.auth.token,
      });
    }
  });
}
</script>

<template>
  <div class="card-layout">
    <Loading v-if="loading" />
    <Card v-else-if="remoteWorld" class="md:min-w-150! md:max-w-1/2">
      <CardHeader>
        <CardTitle>{{ t('join-game') }}</CardTitle>
      </CardHeader>

      <CardContent class="relative size-full overflow-auto flex flex-col">
        <h1 class="font-semibold text-lg text-center">{{ remoteWorld.name }}</h1>
        <div class="grid grid-cols-3 gap-x-2 md:gap-x-4 gap-y-0 px-2 md:px-4">
          <Box :label="t('round')" :content="remoteWorld.currentRound" />
          <Box :label="t('active-players')" :content="remoteWorld.activePlayers" />
          <Box :label="t('total-players')" :content="remoteWorld.totalPlayers" />
          <Box :label="t('created-by')" :content="remoteWorld.createdBy" />
          <Box :label="t('created-at')" :content="formatToday(remoteWorld.createdAtDate)" />
          <Box :label="t('updated-at')" :content="formatToday(remoteWorld.updatedAtDate)" />
        </div>

        <div v-if="remoteWorld.description" class="w-full flex justify-center items-center py-2">
          <span class="md:max-w-6/8 text-sm md:text-base text-center text-muted-foreground break-all wrap-anywhere">
            {{ remoteWorld.description }}
          </span>
        </div>
      </CardContent>

      <CardFooter class="w-full flex flex-col gap-4!">
        <Input
          v-if="remoteWorld.hasPassword"
          v-model="worldPassword"
          type="password"
          :placeholder="t('password')"
          :disabled="locked"
          :minlength="3"
          :maxlength="50"
          class="w-full md:max-w-2/3"
          @keydown.enter="join"
        />

        <div class="w-full md:max-w-1/2 grid grid-cols-2 gap-2">
          <ButtonSpinner :loading="locked" :disabled="!canJoin || locked" @click="join">
            {{ t('join') }}
          </ButtonSpinner>
          <Button variant="secondary" :disabled="locked" @click="() => router.back()">
            <span>{{ t('cancel') }}</span>
          </Button>
        </div>
      </CardFooter>
    </Card>
  </div>
</template>
