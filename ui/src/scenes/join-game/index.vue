<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { joinGame } from '@/core/game';
import { localRef } from '@tb-dev/vue';
import { isPlayerOptions } from '@/lib/schema';
import { SocketAddrV4 } from '@/lib/net/addr-v4';
import enUS from '@/locale/en-US/scenes/join-game.json';
import ptBR from '@/locale/pt-BR/scenes/join-game.json';
import type { Option, WritablePartial } from '@tb-dev/utils';
import {
  Button,
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
  Input,
  Label,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const player = localRef<WritablePartial<PlayerOptions>>('join-game:player', {
  id: null,
});

const server = localRef<Option<string>>('join-game:server', null);
const serverAddr = computed(() => SocketAddrV4.tryParse(server.value));

const loading = ref(false);
const canJoin = computed(() => {
  return isPlayerOptions(player.value) && serverAddr.value;
});

async function join() {
  if (canJoin.value && !loading.value) {
    try {
      loading.value = true;
      await joinGame({
        player: player.value as PlayerOptions,
        serverAddr: serverAddr.value!,
      });
    } finally {
      loading.value = false;
    }
  }
}
</script>

<template>
  <div class="flex size-full flex-col items-center justify-center gap-2">
    <Card class="p-2 sm:min-w-80">
      <CardHeader class="pt-4">
        <CardTitle>
          <span class="text-xl">{{ t('join-game') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="flex flex-col gap-4 px-4">
        <Label>
          <span>{{ t('player-name') }}</span>
          <Input
            v-model.trim="player.id"
            type="text"
            :disabled="loading"
            :minlength="1"
            :maxlength="20"
          />
        </Label>
        <Label>
          <span>{{ t('server') }}</span>
          <Input
            v-model.trim="server"
            type="text"
            :disabled="loading"
            :minlength="1"
            :maxlength="50"
          />
        </Label>
      </CardContent>

      <CardFooter class="grid grid-cols-2 items-center justify-center gap-2 px-6 pb-6">
        <Button :disabled="!canJoin || loading" @click="join">
          {{ t('join') }}
        </Button>
        <Button variant="secondary">
          <RouterLink :to="{ name: 'home' satisfies Scene }">
            {{ t('cancel') }}
          </RouterLink>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
