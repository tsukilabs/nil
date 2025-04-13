<script setup lang="ts">
import { computed } from 'vue';
import { useLocale } from '@/locale';
import { joinGame } from '@/core/game';
import { isPlayerOptions } from '@/lib/schema';
import { SocketAddrV4 } from '@/lib/net/addr-v4';
import type { Option, WritablePartial } from '@tb-dev/utils';
import { Button, ButtonLink, Card, InputText, localRef } from '@tb-dev/vue';

const { t } = useLocale();

const player = localRef<WritablePartial<PlayerOptions>>('join-game:player', {
  id: null,
});

const server = localRef<Option<string>>('join-game:server', null);
const serverAddr = computed(() => SocketAddrV4.tryParse(server.value));

const canJoin = computed(() => {
  return isPlayerOptions(player.value) && serverAddr.value;
});

async function join() {
  if (canJoin.value) {
    await joinGame({
      player: player.value as PlayerOptions,
      serverAddr: serverAddr.value!,
    });
  }
}
</script>

<template>
  <div class="bg-muted/40 flex size-full flex-col items-center justify-center gap-2">
    <Card class="p-2 sm:min-w-72">
      <template #title>
        <h1 class="text-xl">{{ t('join-game') }}</h1>
      </template>

      <div class="flex flex-col gap-6 px-4 pb-4">
        <div class="flex flex-col gap-4">
          <InputText v-model="player.id" :label="t('player-name')" :max="20" />
          <InputText v-model="server" :label="t('server')" />
        </div>

        <div class="flex items-center justify-center gap-2">
          <Button :disabled="!canJoin" @click="join">{{ t('join') }}</Button>
          <ButtonLink to="home" variant="secondary">{{ t('cancel') }}</ButtonLink>
        </div>
      </div>
    </Card>
  </div>
</template>
