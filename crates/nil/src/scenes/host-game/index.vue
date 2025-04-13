<script setup lang="ts">
import { computed } from 'vue';
import { useLocale } from '@/locale';
import { hostGame } from '@/core/game';
import type { WritablePartial } from '@tb-dev/utils';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import { Button, ButtonLink, Card, InputNumber, InputText, localRef } from '@tb-dev/vue';

const { t } = useLocale();

const world = localRef<WritablePartial<WorldOptions>>('host-game:world', {
  name: null,
  size: 100,
});

const player = localRef<WritablePartial<PlayerOptions>>('host-game:player', {
  id: null,
});

const canHost = computed(() => {
  return isWorldOptions(world.value) && isPlayerOptions(player.value);
});

async function host() {
  if (canHost.value) {
    await hostGame({
      player: player.value as PlayerOptions,
      world: world.value as WorldOptions,
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
          <InputText v-model="world.name" :label="t('world-name')" :max="30" />
          <InputNumber v-model="world.size" :label="t('world-size')" :min="10" :max="255" />
          <InputText v-model="player.id" :label="t('player-name')" :max="20" />
        </div>

        <div class="flex items-center justify-center gap-2">
          <Button :disabled="!canHost" @click="host">{{ t('host') }}</Button>
          <ButtonLink to="home" variant="secondary">{{ t('cancel') }}</ButtonLink>
        </div>
      </div>
    </Card>
  </div>
</template>
