<script setup lang="ts">
import { Game } from '@/core/game';
import { computed, ref } from 'vue';
import { useLocale } from '@/locale';
import { isPlayerOptions } from '@/lib/schema';
import { SocketAddrV4 } from '@/lib/net/addr-v4';
import type { PlayerOptions } from '@/types/player';
import { Button, ButtonLink, Card, InputText, Label } from '@/components';

const { t } = useLocale();
const game = Game.use();

const player = ref<PlayerOptions>({
  id: '',
});

const server = ref('');
const serverAddr = computed(() => SocketAddrV4.tryParse(server.value));

const canJoin = computed(() => {
  return isPlayerOptions(player.value) && serverAddr.value;
});

async function join() {
  if (canJoin.value && serverAddr.value) {
    await game.join(serverAddr.value, player.value);
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
          <Label>
            <span>{{ t('player-name') }}</span>
            <InputText v-model="player.id" />
          </Label>
          <Label>
            <span>{{ t('server') }}</span>
            <InputText v-model="server" />
          </Label>
        </div>

        <div class="flex items-center justify-center gap-2">
          <Button :disabled="!canJoin" @click="() => join()">{{ t('join') }}</Button>
          <ButtonLink to="home" variant="secondary">{{ t('cancel') }}</ButtonLink>
        </div>
      </div>
    </Card>
  </div>
</template>
