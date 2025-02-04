<script setup lang="ts">
import { computed, ref } from 'vue';
import { useLocale } from '@/locale';
import { World } from '@/core/world';
import { SocketAddrV4 } from '@/lib/net/addr';
import { isPlayerConfig } from '@/lib/schema';
import type { PlayerConfig } from '@/types/player';
import { Button, Card, InputText, Label } from '@/components';

const { t } = useLocale();
const world = World.use();

const player = ref<PlayerConfig>({
  name: '',
});

const server = ref('');
const serverAddr = computed(() => SocketAddrV4.tryParse(server.value));

const canJoin = computed(() => {
  return isPlayerConfig(player.value) && serverAddr.value;
});

async function join() {
  if (canJoin.value && serverAddr.value) {
    await world.join(serverAddr.value, player.value);
  }
}
</script>

<template>
  <div class="bg-muted/40 flex size-full flex-col items-center justify-center gap-2">
    <Card class="p-2 sm:min-w-72">
      <template #title>
        <h1 class="text-xl">{{ t('join-game') }}</h1>
      </template>

      <div class="flex flex-col gap-6">
        <div class="flex flex-col gap-4">
          <Label>
            <span>{{ t('name') }}</span>
            <InputText v-model="player.name" />
          </Label>
          <Label>
            <span>{{ t('server') }}</span>
            <InputText v-model="server" />
          </Label>
        </div>

        <div class="flex items-center justify-center gap-2">
          <Button :disabled="!canJoin" @click="() => join()">{{ t('join') }}</Button>
          <Button variant="secondary" @click="() => $go('home')">{{ t('cancel') }}</Button>
        </div>
      </div>
    </Card>
  </div>
</template>
