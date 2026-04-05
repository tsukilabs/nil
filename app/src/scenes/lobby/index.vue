<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { formatDate } from 'date-fns';
import { LockIcon } from '@lucide/vue';
import Loading from '@/components/Loading.vue';
import { throttle } from 'es-toolkit/function';
import type { WorldId } from '@/types/core/world';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { useToken } from '@/composables/auth/useToken';
import { onKeyDown, useBreakpoints } from '@tb-dev/vue';
import { go, QUERY_JOIN_REMOTE_GAME_WORLD_ID } from '@/router';
import { Card, CardContent, CardHeader, CardTitle } from '@ui/card';
import { useRemoteWorlds } from '@/composables/world/useRemoteWorlds';
import { useRemoteWorldLimit } from '@/composables/world/useRemoteWorldLimit';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@ui/table';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { sm, md, xl } = useBreakpoints();

const { remoteWorlds, loading, load } = useRemoteWorlds();

const worldLimit = useRemoteWorldLimit();

const { playerId, isTokenValid } = useToken();

const canHost = computed(() => {
  return (
    isTokenValid.value &&
    remoteWorlds.value.length < (worldLimit.value.global ?? 0) &&
    countCurrentPlayerWorlds() < (worldLimit.value.perUser ?? 0)
  );
});

const someHasPassword = computed(() => {
  return remoteWorlds.value.some((world) => world.hasPassword);
});

if (__DESKTOP__) {
  onKeyDown('F5', throttle(load, 1000));
}

async function goToJoinRemoteGameScene(id: WorldId) {
  await go('join-remote-game', { query: { [QUERY_JOIN_REMOTE_GAME_WORLD_ID]: id } });
}

function countCurrentPlayerWorlds() {
  return remoteWorlds.value.reduce((acc, world) => {
    if (world.createdBy === playerId.value) ++acc;
    return acc;
  }, 0);
}
</script>

<template>
  <div class="flex size-full overflow-hidden p-1 sm:p-2 md:p-4">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span>{{ t('lobby') }}</span>
            <div class="grid grid-cols-2 gap-2">
              <Button
                variant="default"
                :size="sm ? 'default' : 'sm'"
                :disabled="!canHost"
                class="md:px-4 xl:px-6 2xl:px-8"
                @click="() => go('host-remote-game')"
              >
                <span>{{ t('host') }}</span>
              </Button>
              <Button
                variant="secondary"
                :size="sm ? 'default' : 'sm'"
                class="md:px-4 xl:px-6 2xl:px-8"
                @click="() => go('home')"
              >
                <span>{{ t('leave') }}</span>
              </Button>
            </div>
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full overflow-auto px-2 py-0">
        <Loading v-if="loading" />
        <Table v-else-if="remoteWorlds.length > 0" class="w-full min-w-max">
          <TableHeader>
            <TableRow class="hover:bg-card">
              <TableHead v-if="someHasPassword"></TableHead>
              <TableHead>{{ t('name') }}</TableHead>
              <TableHead>{{ t('round') }}</TableHead>
              <TableHead v-if="md">{{ t('active-players') }}</TableHead>
              <TableHead>{{ md ? t('total-players') : t('player', 2) }}</TableHead>
              <TableHead v-if="xl">{{ t('size') }}</TableHead>
              <TableHead>{{ t('created-by') }}</TableHead>
              <TableHead v-if="xl">{{ t('created-at') }}</TableHead>
              <TableHead v-if="xl">{{ t('updated-at') }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow
              v-for="world of remoteWorlds"
              :key="world.config.id"
              role="link"
              tabindex="0"
              class="cursor-pointer"
              @click="() => goToJoinRemoteGameScene(world.config.id)"
            >
              <TableCell v-if="someHasPassword">
                <LockIcon v-if="world.hasPassword" class="size-4" />
              </TableCell>
              <TableCell>
                <span>{{ world.config.name }}</span>
              </TableCell>
              <TableCell>
                <span>{{ world.currentRound }}</span>
              </TableCell>
              <TableCell v-if="md">
                <span>{{ world.activePlayers }}</span>
              </TableCell>
              <TableCell>
                <span>{{ world.totalPlayers }}</span>
              </TableCell>
              <TableCell v-if="xl">
                <span>{{ world.continentSize }}</span>
              </TableCell>
              <TableCell>
                <span>{{ world.createdBy }}</span>
              </TableCell>
              <TableCell v-if="xl">
                <span>{{ formatDate(world.createdAtDate, 'dd/MM/yyyy HH:mm') }}</span>
              </TableCell>
              <TableCell v-if="xl">
                <span>{{ formatDate(world.updatedAtDate, 'dd/MM/yyyy HH:mm') }}</span>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
