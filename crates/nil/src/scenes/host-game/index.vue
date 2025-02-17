<script setup lang="ts">
import { Game } from '@/core/game';
import { computed, ref } from 'vue';
import { useLocale } from '@/locale';
import type { WorldOptions } from '@/types/world';
import type { PlayerOptions } from '@/types/player';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import { Button, ButtonLink, Card, InputNumber, InputText, Label } from '@/components';

const { t } = useLocale();
const game = Game.use();

const world = ref<WorldOptions>({
  size: 100,
});

const player = ref<PlayerOptions>({
  id: '',
});

const canHost = computed(() => {
  return isWorldOptions(world.value) && isPlayerOptions(player.value);
});

async function host() {
  if (canHost.value) {
    await game.host({
      player: player.value,
      world: world.value,
    });
  }
}
</script>

<template>
  <div class="bg-muted/40 flex size-full flex-col items-center justify-center gap-2">
    <Card class="p-2 sm:min-w-72">
      <template #title>
        <h1 class="text-xl">{{ t('host-game') }}</h1>
      </template>

      <div class="flex flex-col gap-6 px-4 pb-4">
        <div class="flex flex-col gap-4">
          <Label>
            <span>{{ t('player-name') }}</span>
            <InputText v-model="player.id" />
          </Label>
          <Label>
            <span>{{ t('world-size') }}</span>
            <InputNumber v-model="world.size" :min="10" :max="255" />
          </Label>
        </div>

        <div class="flex items-center justify-center gap-2">
          <Button :disabled="!canHost" @click="() => host()">{{ t('host') }}</Button>
          <ButtonLink to="home" variant="secondary">{{ t('cancel') }}</ButtonLink>
        </div>
      </div>
    </Card>
  </div>
</template>
