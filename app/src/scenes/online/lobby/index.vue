<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { throttle } from 'es-toolkit';
import { formatToday } from '@/lib/date';
import { LockIcon } from 'lucide-vue-next';
import Loading from '@/components/Loading.vue';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { onKeyDown, useBreakpoints } from '@tb-dev/vue';
import { watchToken } from '@/composables/online/watchToken';
import { useRemoteWorlds } from '@/composables/useRemoteWorlds';
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { sm, md, xl } = useBreakpoints();

const { remoteWorlds, loading, load } = useRemoteWorlds();

const someHasPassword = computed(() => {
  return remoteWorlds.value.some((world) => world.hasPassword);
});

if (__DESKTOP__) {
  onKeyDown('F5', throttle(load, 1000));
}

watchToken('sign-in');
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
                class="md:px-4 xl:px-6 2xl:px-8"
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

      <CardContent class="relative size-full overflow-auto px-2 py-0">
        <Loading v-if="loading" />
        <Table v-else-if="remoteWorlds.length > 0">
          <TableHeader>
            <TableRow class="hover:bg-card">
              <TableHead v-if="someHasPassword"></TableHead>
              <TableHead>{{ t('name') }}</TableHead>
              <TableHead>{{ t('round') }}</TableHead>
              <TableHead v-if="md">{{ t('active-players') }}</TableHead>
              <TableHead>{{ md ? t('total-players') : t('player', 2) }}</TableHead>
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
            >
              <TableCell v-if="someHasPassword">
                <LockIcon v-if="world.hasPassword" class="size-4" />
              </TableCell>
              <TableCell>
                <span class="break-all wrap-anywhere">{{ world.config.name }}</span>
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
              <TableCell>
                <span class="break-all wrap-anywhere">{{ world.createdBy }}</span>
              </TableCell>
              <TableCell v-if="xl">
                <span>{{ formatToday(world.createdAtDate) }}</span>
              </TableCell>
              <TableCell v-if="xl">
                <span>{{ formatToday(world.updatedAtDate) }}</span>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
