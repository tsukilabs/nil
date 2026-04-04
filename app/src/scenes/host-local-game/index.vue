<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { computed } from 'vue';
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { Switch } from '@ui/switch';
import { useRouter } from 'vue-router';
import { hostLocalGame } from '@/core/game';
import { useSettings } from '@/stores/settings';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import { localRef, useBreakpoints, useMutex } from '@tb-dev/vue';
import InputWorldName from '@/components/form/InputWorldName.vue';
import InputWorldSize from '@/components/form/InputWorldSize.vue';
import InputPlayerName from '@/components/form/InputPlayerName.vue';
import SliderBotDensity from '@/components/form/SliderBotDensity.vue';
import SliderWorldSpeed from '@/components/form/SliderWorldSpeed.vue';
import type { WithPartialNullish, WritablePartial } from '@tb-dev/utils';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@ui/card';
import SliderBotAdvancedStartRatio from '@/components/form/SliderBotAdvancedStartRatio.vue';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const settings = useSettings();

const { md } = useBreakpoints();

const worldOptions = localRef<WritablePartial<WorldOptions>>(
  key('world'),
  {
    name: null,
    size: __CONSTS__.continentSizeDefault,
    locale: settings.general.locale,
    allowCheats: false,
    speed: __CONSTS__.worldSpeedDefault,
    botDensity: __CONSTS__.botDensityDefault,
    botAdvancedStartRatio: __CONSTS__.botAdvancedStartRatioDefault,
  } satisfies WithPartialNullish<WorldOptions, 'name'>,
);

const playerOptions = localRef<WritablePartial<PlayerOptions>>(
  key('player'),
  {
    id: null,
  } satisfies WithPartialNullish<PlayerOptions, 'id'>,
);

const { locked, lock } = useMutex();
const isValidPlayer = computed(() => isPlayerOptions(playerOptions.value));
const isValidWorld = computed(() => isWorldOptions(worldOptions.value));
const canHost = computed(() => isValidPlayer.value && isValidWorld.value);

async function host() {
  worldOptions.value.locale = settings.general.locale;
  await lock(async () => {
    if (isPlayerOptions(playerOptions.value) && isWorldOptions(worldOptions.value)) {
      await hostLocalGame({
        playerOptions: playerOptions.value,
        worldOptions: worldOptions.value,
      });
    }
  });
}

function key(name: string) {
  return `host-local-game:${name}`;
}
</script>

<template>
  <div :class="md ? 'card-layout' : 'game-layout'">
    <Card class="max-md:size-full overflow-hidden">
      <CardHeader>
        <CardTitle>{{ t('host-game') }}</CardTitle>
      </CardHeader>

      <CardContent class="size-full flex flex-col gap-2 max-md:px-2 overflow-x-hidden overflow-y-auto">
        <InputWorldName v-model="worldOptions.name" :disabled="locked" />
        <InputWorldSize v-model="worldOptions.size" :disabled="locked" />
        <InputPlayerName v-model="playerOptions.id" :disabled="locked" />
        <SliderWorldSpeed v-model="worldOptions" :disabled="locked" />
        <SliderBotDensity v-model="worldOptions" :disabled="locked" />
        <SliderBotAdvancedStartRatio v-model="worldOptions" :disabled="locked" />

        <div class="flex items-center justify-center py-1">
          <Label>
            <Switch v-model="worldOptions.allowCheats" :disabled="locked" />
            <span>{{ t('allow-cheats') }}</span>
          </Label>
        </div>
      </CardContent>

      <CardFooter class="w-full grid grid-cols-3 gap-2">
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
