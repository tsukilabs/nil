<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { localRef } from '@tb-dev/vue';
import type { WritablePartial } from '@tb-dev/utils';
import { hostGame, hostSavedGame } from '@/core/game';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import { open as pickFile } from '@tauri-apps/plugin-dialog';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import {
  Button,
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
  Checkbox,
  Input,
  Label,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
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
  if (isValidPlayer.value && isValidWorld.value && !loading.value) {
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
  if (isValidPlayer.value && !loading.value) {
    try {
      loading.value = true;
      const path = await pickFile({
        directory: false,
        filters: [{ extensions: ['nil'], name: 'NIL' }],
        multiple: false,
      });

      if (path) {
        await hostSavedGame({
          path,
          player: player.value as PlayerOptions,
        });
      }
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
          <span class="text-xl">{{ t('host-game') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="flex flex-col gap-4 px-4">
        <Label>
          <span>{{ t('world-name') }}</span>
          <Input
            v-model="world.name"
            type="text"
            :disabled="loading"
            :minlength="1"
            :maxlength="30"
          />
        </Label>

        <Label>
          <span>{{ t('world-size') }}</span>
          <NumberField
            v-model="world.size"
            :disabled="loading"
            :min="100"
            :max="200"
            :step="10"
            class="w-full"
          >
            <NumberFieldContent>
              <NumberFieldDecrement />
              <NumberFieldInput />
              <NumberFieldIncrement />
            </NumberFieldContent>
          </NumberField>
        </Label>

        <Label>
          <span>{{ t('player-name') }}</span>
          <Input
            v-model="player.id"
            type="text"
            :disabled="loading"
            :minlength="1"
            :maxlength="20"
          />
        </Label>

        <div class="flex items-center justify-center py-1">
          <Label>
            <Checkbox v-model="world.allowCheats" :disabled="loading" />
            <span>{{ t('allow-cheats') }}</span>
          </Label>
        </div>
      </CardContent>

      <CardFooter class="grid grid-cols-3 items-center justify-center gap-2 px-6 pb-6">
        <Button :disabled="loading || !canHost" @click="host">
          <span>{{ t('host') }}</span>
        </Button>
        <Button variant="secondary" :disabled="loading || !isValidPlayer" @click="hostSaved">
          <span>{{ t('load') }}</span>
        </Button>
        <Button to="home" variant="secondary">
          <RouterLink :to="{ name: 'home' as Scene }">
            {{ t('cancel') }}
          </RouterLink>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
