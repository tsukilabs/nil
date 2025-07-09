<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { until } from '@vueuse/core';
import { joinGame } from '@/core/game';
import { localRef } from '@tb-dev/vue';
import { isPlayerOptions } from '@/lib/schema';
import { SocketAddrV4 } from '@/lib/net/addr-v4';
import type { Option, WritablePartial } from '@tb-dev/utils';
import { Button, ButtonLink, Card, InputText, Label } from '@tb-dev/vue-components';

const { t } = useI18n();

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
  if (canJoin.value) {
    await until(loading).not.toBeTruthy();
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
  <div class="bg-muted/40 flex size-full flex-col items-center justify-center gap-2">
    <Card class="p-2 sm:min-w-72">
      <template #title>
        <span class="text-xl">{{ t('join-game') }}</span>
      </template>

      <div class="flex flex-col gap-6 px-4 pb-4">
        <div class="flex flex-col gap-4">
          <Label>
            <span>{{ t('player-name') }}</span>
            <InputText v-model="player.id" :disabled="loading" :min="1" :max="20" />
          </Label>
          <Label>
            <span>{{ t('server') }}</span>
            <InputText v-model="server" :disabled="loading" :min="1" :max="50" />
          </Label>
        </div>

        <div class="grid grid-cols-2 items-center justify-center gap-2 px-4">
          <Button :disabled="!canJoin || loading" @click="join">{{ t('join') }}</Button>
          <ButtonLink to="home" variant="secondary">{{ t('cancel') }}</ButtonLink>
        </div>
      </div>
    </Card>
  </div>
</template>
