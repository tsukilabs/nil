<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { hostGame } from '@/core/game';
import { useRouter } from 'vue-router';
import { localRef, useMutex } from '@tb-dev/vue';
import type { WritablePartial } from '@tb-dev/utils';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
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

const router = useRouter();

const world = localRef<WritablePartial<WorldOptions>>('host-game:world', {
  name: null,
  size: 100,
  allowCheats: false,
});

const player = localRef<WritablePartial<PlayerOptions>>('host-game:player', {
  id: null,
});

const { locked, lock } = useMutex();
const isValidPlayer = computed(() => isPlayerOptions(player.value));
const isValidWorld = computed(() => isWorldOptions(world.value));
const canHost = computed(() => isValidPlayer.value && isValidWorld.value);

async function host() {
  await lock(async () => {
    if (isPlayerOptions(player.value) && isWorldOptions(world.value)) {
      await hostGame(player.value, world.value);
    }
  });
}

async function goToLoadGameScene() {
  await go('load-game', {
    query: { playerId: player.value.id },
  });
}
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardHeader>
        <CardTitle>{{ t('host-game') }}</CardTitle>
      </CardHeader>

      <CardContent>
        <Label>
          <span>{{ t('world-name') }}</span>
          <Input
            v-model.trim="world.name"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="30"
          />
        </Label>

        <Label>
          <span>{{ t('world-size') }}</span>
          <NumberField
            v-model="world.size"
            :disabled="locked"
            :min="100"
            :max="200"
            :step="10"
            class="w-full"
          >
            <NumberFieldContent>
              <NumberFieldDecrement />
              <NumberFieldInput class="dark:bg-input/40" />
              <NumberFieldIncrement />
            </NumberFieldContent>
          </NumberField>
        </Label>

        <Label>
          <span>{{ t('player-name') }}</span>
          <Input
            v-model.trim="player.id"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="20"
          />
        </Label>

        <div class="flex items-center justify-center py-1">
          <Label>
            <Checkbox v-model="world.allowCheats" :disabled="locked" />
            <span>{{ t('allow-cheats') }}</span>
          </Label>
        </div>
      </CardContent>

      <CardFooter class="grid grid-cols-3">
        <Button :disabled="locked || !canHost" @click="host">
          <span>{{ t('host') }}</span>
        </Button>

        <Button
          variant="secondary"
          :disabled="locked || !isValidPlayer"
          role="link"
          tabindex="0"
          @click="goToLoadGameScene"
        >
          <span>{{ t('load') }}</span>
        </Button>

        <Button variant="secondary" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
