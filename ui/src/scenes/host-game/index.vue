<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { until } from '@vueuse/core';
import { localRef } from '@tb-dev/vue';
import type { WritablePartial } from '@tb-dev/utils';
import { hostGame, hostSavedGame } from '@/core/game';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import { open as pickFile } from '@tauri-apps/plugin-dialog';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import {
  Button,
  ButtonLink,
  Card,
  Checkbox,
  InputNumber,
  InputText,
  Label,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const world = localRef<WritablePartial<WorldOptions>>('host-game:world', {
  name: null,
  size: 100,
  allowCheats: false,
});

const player = localRef<WritablePartial<PlayerOptions>>('host-game:player', {
  id: null,
});

const loading = ref(false);
const isValidPlayer = computed(() => isPlayerOptions(player.value));
const isValidWorld = computed(() => isWorldOptions(world.value));
const canHost = computed(() => isValidPlayer.value && isValidWorld.value);

async function host() {
  if (isValidPlayer.value && isValidWorld.value) {
    await until(loading).not.toBeTruthy();
    try {
      loading.value = true;
      await hostGame({
        player: player.value as PlayerOptions,
        world: world.value as WorldOptions,
      });
    } finally {
      loading.value = false;
    }
  }
}

async function hostSaved() {
  const path = await pickFile({
    directory: false,
    filters: [{ extensions: ['nil'], name: 'NIL' }],
    multiple: false,
  });

  if (path && isValidPlayer.value) {
    await until(loading).not.toBeTruthy();
    try {
      loading.value = true;
      await hostSavedGame({
        path,
        player: player.value as PlayerOptions,
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
        <span class="text-xl">{{ t('host-game') }}</span>
      </template>

      <div class="flex flex-col gap-6 px-4 pb-4">
        <div class="flex flex-col gap-4">
          <Label>
            <span>{{ t('world-name') }}</span>
            <InputText v-model="world.name" :disabled="loading" :min="1" :max="30" />
          </Label>
          <Label>
            <span>{{ t('world-size') }}</span>
            <InputNumber v-model="world.size" :disabled="loading" :min="10" :max="200" :step="2" />
          </Label>
          <Label>
            <span>{{ t('player-name') }}</span>
            <InputText v-model="player.id" :disabled="loading" :min="1" :max="20" />
          </Label>

          <div class="flex items-center justify-center py-1">
            <Label>
              <Checkbox v-model="world.allowCheats" />
              <span>{{ t('allow-cheats') }}</span>
            </Label>
          </div>
        </div>

        <div class="grid grid-cols-3 items-center justify-center gap-2 px-4">
          <Button :disabled="loading || !canHost" @click="host">
            <span>{{ t('host') }}</span>
          </Button>
          <Button variant="secondary" :disabled="loading || !isValidPlayer" @click="hostSaved">
            <span>{{ t('load') }}</span>
          </Button>
          <ButtonLink to="home" variant="secondary">
            <span>{{ t('cancel') }}</span>
          </ButtonLink>
        </div>
      </div>
    </Card>
  </div>
</template>
