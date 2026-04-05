<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';
import { Button } from '@ui/button';
import { Switch } from '@ui/switch';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { hostRemoteGame } from '@/core/game';
import { toMerged } from 'es-toolkit/object';
import { useSettings } from '@/stores/settings';
import type { WorldOptions } from '@/types/core/world';
import enUS_online from '@/locale/en-US/scenes/online.json';
import ptBR_online from '@/locale/pt-BR/scenes/online.json';
import enUS_hostGame from '@/locale/en-US/scenes/host-game.json';
import ptBR_hostGame from '@/locale/pt-BR/scenes/host-game.json';
import { localRef, useBreakpoints, useMutex } from '@tb-dev/vue';
import ButtonSpinner from '@/components/button/ButtonSpinner.vue';
import InputWorldName from '@/components/form/InputWorldName.vue';
import InputWorldSize from '@/components/form/InputWorldSize.vue';
import SliderBotDensity from '@/components/form/SliderBotDensity.vue';
import SliderWorldSpeed from '@/components/form/SliderWorldSpeed.vue';
import { isValidNullishPassword, isWorldOptions } from '@/lib/schema';
import type { WithPartialNullish, WritablePartial } from '@tb-dev/utils';
import InputWorldPassword from '@/components/form/InputWorldPassword.vue';
import SliderRoundDuration from '@/components/form/SliderRoundDuration.vue';
import SliderWorldUnitSpeed from '@/components/form/SliderWorldUnitSpeed.vue';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@ui/card';
import TextareaWorldDescription from '@/components/form/TextareaWorldDescription.vue';
import SliderBotAdvancedStartRatio from '@/components/form/SliderBotAdvancedStartRatio.vue';

const { t } = useI18n({
  messages: {
    'en-US': toMerged(enUS_hostGame, enUS_online),
    'pt-BR': toMerged(ptBR_hostGame, ptBR_online),
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
    unitSpeed: __CONSTS__.worldUnitSpeedDefault,
    botDensity: __CONSTS__.botDensityDefault,
    botAdvancedStartRatio: __CONSTS__.botAdvancedStartRatioDefault,
  } satisfies WithPartialNullish<WorldOptions, 'name'>,
);

const worldPassword = ref<Option<string>>();
const description = ref<Option<string>>();

const roundDuration = localRef(key('round-duration'), __CONSTS__.roundDurationDefault);
const isRoundDurationEnabled = localRef(key('round-duration-enabled'), false);

const { locked, lock } = useMutex();
const isValidWorld = computed(() => isWorldOptions(worldOptions.value));
const canHost = computed(() => {
  return (
    isValidWorld.value &&
    Boolean(settings.auth.token) &&
    isValidNullishPassword(worldPassword.value)
  );
});

async function host() {
  worldOptions.value.locale = settings.general.locale;
  await lock(async () => {
    if (isWorldOptions(worldOptions.value) && settings.auth.token) {
      await hostRemoteGame({
        worldOptions: worldOptions.value,
        worldPassword: worldPassword.value,
        worldDescription: description.value,
        authorizationToken: settings.auth.token,
        roundDuration: isRoundDurationEnabled.value ? roundDuration.value : null,
      });
    }
  });
}

function key(name: string) {
  return `host-remote-game:${name}`;
}
</script>

<template>
  <div :class="md ? 'card-layout' : 'game-layout'">
    <Card class="max-md:size-full md:min-w-150! md:max-w-1/2 md:max-h-[95%] overflow-hidden">
      <CardHeader>
        <CardTitle>{{ t('host-game') }}</CardTitle>
      </CardHeader>

      <CardContent class="card-form">
        <InputWorldName v-model="worldOptions.name" :disabled="locked" />
        <InputWorldSize v-model="worldOptions.size" :disabled="locked" />
        <InputWorldPassword v-model="worldPassword" :disabled="locked" />
        <TextareaWorldDescription v-model="description" :disabled="locked" />
        <SliderWorldSpeed v-model="worldOptions" :disabled="locked" />
        <SliderWorldUnitSpeed v-model="worldOptions" :disabled="locked" />
        <SliderBotDensity v-model="worldOptions" :disabled="locked" />
        <SliderBotAdvancedStartRatio v-model="worldOptions" :disabled="locked" />
        <SliderRoundDuration
          v-model:duration="roundDuration"
          v-model:enabled="isRoundDurationEnabled"
          :disabled="locked"
        />

        <div class="flex items-center justify-center py-1">
          <Label>
            <Switch v-model="worldOptions.allowCheats" :disabled="locked" />
            <span>{{ t('allow-cheats') }}</span>
          </Label>
        </div>
      </CardContent>

      <CardFooter class="w-full flex">
        <div class="w-full md:max-w-1/2 grid grid-cols-2 gap-2">
          <ButtonSpinner :loading="locked" :disabled="locked || !canHost" @click="host">
            <span>{{ t('host') }}</span>
          </ButtonSpinner>

          <Button variant="secondary" :disabled="locked" @click="() => router.back()">
            <span>{{ t('cancel') }}</span>
          </Button>
        </div>
      </CardFooter>
    </Card>
  </div>
</template>
