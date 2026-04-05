<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Input } from '@ui/input';
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { computed, ref } from 'vue';
import { formatDate } from 'date-fns';
import * as commands from '@/commands';
import { useRouter } from 'vue-router';
import { Trash2Icon } from '@lucide/vue';
import { joinRemoteGame } from '@/core/game';
import Loading from '@/components/Loading.vue';
import { isValidPassword } from '@/lib/schema';
import { useRouteQuery } from '@vueuse/router';
import { useSettings } from '@/stores/settings';
import type { WorldId } from '@/types/core/world';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { useToken } from '@/composables/auth/useToken';
import { useBreakpoints, useMutex } from '@tb-dev/vue';
import ButtonIcon from '@/components/button/ButtonIcon.vue';
import { go, QUERY_JOIN_REMOTE_GAME_WORLD_ID } from '@/router';
import ButtonSpinner from '@/components/button/ButtonSpinner.vue';
import { useRemoteWorld } from '@/composables/world/useRemoteWorld';
import { Table, TableBody, TableCell, TableHead, TableRow } from '@ui/table';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@ui/card';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const worldId = useRouteQuery<Option<WorldId>>(QUERY_JOIN_REMOTE_GAME_WORLD_ID, null);

const settings = useSettings();

const { md } = useBreakpoints();

const { remoteWorld, loading } = useRemoteWorld(worldId);

const { playerId } = useToken();

const worldPassword = ref<Option<string>>();

const { locked, lock } = useMutex();
const canJoin = computed(() => {
  return (
    Boolean(remoteWorld.value) &&
    Boolean(settings.auth.token) &&
    (!remoteWorld.value?.hasPassword || isValidPassword(worldPassword.value))
  );
});

async function joinGame() {
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

async function deleteGame() {
  await lock(async () => {
    if (remoteWorld.value && settings.auth.token) {
      await commands.deleteRemoteWorld(remoteWorld.value.id);
      await go('lobby');
    }
  });
}
</script>

<template>
  <div :class="md ? 'card-layout' : 'game-layout'">
    <Loading v-if="loading" />
    <Card v-else-if="remoteWorld" class="max-md:size-full md:min-w-150! md:max-w-1/2 md:max-h-9/10 overflow-hidden">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span class="w-full">{{ t('join-game') }}</span>
            <div class="flex items-center justify-center">
              <ButtonIcon
                v-if="remoteWorld.createdBy === playerId"
                variant="outline"
                :size="md ? 'icon' : 'icon-sm'"
                :icon="Trash2Icon"
                :disabled="locked"
                @click="deleteGame"
              />
            </div>
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full overflow-auto">
        <Table class="w-full min-w-max">
          <TableBody>
            <TableRow class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('name') }}</TableHead>
              <TableCell class="w-full wrap-anywhere whitespace-normal">
                {{ remoteWorld.name }}
              </TableCell>
            </TableRow>

            <TableRow class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('round') }}</TableHead>
              <TableCell class="w-full">{{ remoteWorld.currentRound }}</TableCell>
            </TableRow>

            <TableRow class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('active-players') }}</TableHead>
              <TableCell class="w-full">{{ remoteWorld.activePlayers }}</TableCell>
            </TableRow>

            <TableRow class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('total-players') }}</TableHead>
              <TableCell class="w-full">{{ remoteWorld.totalPlayers }}</TableCell>
            </TableRow>

            <TableRow class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('created-by') }}</TableHead>
              <TableCell class="w-full wrap-anywhere whitespace-normal">
                {{ remoteWorld.createdBy }}
              </TableCell>
            </TableRow>

            <TableRow class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('created-at') }}</TableHead>
              <TableCell class="w-full">
                {{ formatDate(remoteWorld.createdAtDate, 'dd/MM/yyyy HH:mm') }}
              </TableCell>
            </TableRow>

            <TableRow class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('updated-at') }}</TableHead>
              <TableCell class="w-full">
                {{ formatDate(remoteWorld.updatedAtDate, 'dd/MM/yyyy HH:mm') }}
              </TableCell>
            </TableRow>

            <TableRow v-if="remoteWorld.description" class="hover:bg-card">
              <TableHead class="max-w-max pr-4">{{ t('world-description') }}</TableHead>
              <TableCell class="w-full wrap-anywhere whitespace-normal">
                {{ remoteWorld.description }}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
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
          @keydown.enter="joinGame"
        />

        <div class="w-full md:max-w-1/2 grid grid-cols-2 gap-2">
          <ButtonSpinner :loading="locked" :disabled="!canJoin || locked" @click="joinGame">
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
