<script setup lang="ts">
import { computed, ref } from 'vue';
import { useLocale } from '@/locale';
import { World } from '@/core/world';
import type { PlayerConfig } from '@/types/player';
import { isIp, isPlayerConfig } from '@/lib/schema';
import { Button, Card, InputText, Label } from '@/components';

const world = World.use();

const { t } = useLocale();

const server = ref('');
const player = ref<PlayerConfig>({
  name: '',
});

const canJoin = computed(() => {
  return isPlayerConfig(player.value) && isIp(server.value);
});

async function join() {
  if (canJoin.value) {
    await world.join(server.value, player.value);
  }
}
</script>

<template>
  <div class="bg-muted/40 flex size-full flex-col items-center justify-center gap-2">
    <Card class="p-2 sm:min-w-72">
      <template #title>
        <h1 class="text-xl">{{ t('misc.join-game') }}</h1>
      </template>

      <div class="flex flex-col gap-6">
        <div class="flex flex-col gap-4">
          <Label>
            <span>{{ t('misc.name') }}</span>
            <InputText v-model="player.name" />
          </Label>
          <Label>
            <span>{{ t('misc.server') }}</span>
            <InputText v-model="server" />
          </Label>
        </div>

        <div class="flex items-center justify-center gap-2">
          <Button :disabled="!canJoin" @click="() => join()">{{ t('misc.join') }}</Button>
          <Button variant="secondary" @click="() => $go('home')">{{ t('misc.cancel') }}</Button>
        </div>
      </div>
    </Card>
  </div>
</template>
