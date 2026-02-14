<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { hostLocalGame } from '@/core/game';
import { useSettings } from '@/stores/settings';
import { localRef, useMutex } from '@tb-dev/vue';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import { WorldConfigImpl } from '@/core/model/world-config';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import InputWorldName from '@/components/form/InputWorldName.vue';
import InputWorldSize from '@/components/form/InputWorldSize.vue';
import InputPlayerName from '@/components/form/InputPlayerName.vue';
import SliderBotDensity from '@/components/form/SliderBotDensity.vue';
import type { WithPartialNullish, WritablePartial } from '@tb-dev/utils';
import SliderBotAdvancedStartRatio from '@/components/form/SliderBotAdvancedStartRatio.vue';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle, Checkbox, Label } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const settings = useSettings();

const worldOptions = localRef<WritablePartial<WorldOptions>>(
  'host-local-game:world',
  {
    name: null,
    size: 100,
    locale: settings.locale,
    allowCheats: false,
    botDensity: WorldConfigImpl.DEFAULT_BOT_DENSITY,
    botAdvancedStartRatio: WorldConfigImpl.DEFAULT_BOT_ADVANCED_START_RATIO,
  } satisfies WithPartialNullish<WorldOptions, 'name'>,
);

const playerOptions = localRef<WritablePartial<PlayerOptions>>(
  'host-local-game:player',
  {
    id: null,
  } satisfies WithPartialNullish<PlayerOptions, 'id'>,
);

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
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardHeader>
        <CardTitle>{{ t('host-game') }}</CardTitle>
      </CardHeader>

      <CardContent>
        <InputWorldName v-model="worldOptions.name" :disabled="locked" />
        <InputWorldSize v-model="worldOptions.size" :disabled="locked" />
        <InputPlayerName v-model="playerOptions.id" :disabled="locked" />
        <SliderBotDensity v-model="worldOptions" :disabled="locked" />
        <SliderBotAdvancedStartRatio v-model="worldOptions" :disabled="locked" />

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
          :disabled="locked"
          role="link"
          tabindex="0"
          @click="() => go('load-local-game')"
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
