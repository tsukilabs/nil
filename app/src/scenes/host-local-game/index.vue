<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { hostLocalGame } from '@/core/game';
import { useSettings } from '@/stores/settings';
import { localRef, useMutex } from '@tb-dev/vue';
import type { WritablePartial } from '@tb-dev/utils';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import { go, QUERY_LOAD_LOCAL_GAME_PLAYER_ID } from '@/router';
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

const { t } = useI18n();

const router = useRouter();
const settings = useSettings();

const worldOptions = localRef<WritablePartial<WorldOptions>>('host-local-game:world', {
  name: null,
  size: 100,
  locale: settings.locale,
  allowCheats: false,
});

const playerOptions = localRef<WritablePartial<PlayerOptions>>('host-local-game:player', {
  id: null,
});

const { locked, lock } = useMutex();
const isValidPlayer = computed(() => isPlayerOptions(playerOptions.value));
const isValidWorld = computed(() => isWorldOptions(worldOptions.value));
const canHost = computed(() => isValidPlayer.value && isValidWorld.value);

async function host() {
  worldOptions.value.locale = settings.locale;
  await lock(async () => {
    if (isPlayerOptions(playerOptions.value) && isWorldOptions(worldOptions.value)) {
      await hostLocalGame({
        playerOptions: playerOptions.value,
        worldOptions: worldOptions.value,
      });
    }
  });
}

async function goToLoadGameScene() {
  await go('load-local-game', {
    query: { [QUERY_LOAD_LOCAL_GAME_PLAYER_ID]: playerOptions.value.id },
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
            v-model="worldOptions.name"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="30"
          />
        </Label>

        <Label>
          <span>{{ t('world-size') }}</span>
          <NumberField
            v-model="worldOptions.size"
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
            v-model="playerOptions.id"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="20"
          />
        </Label>

        <div class="flex items-center justify-center py-1">
          <Label>
            <Checkbox v-model="worldOptions.allowCheats" :disabled="locked" />
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

        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
