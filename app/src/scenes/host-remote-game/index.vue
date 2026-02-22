<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { toMerged } from 'es-toolkit';
import { useRouter } from 'vue-router';
import { hostRemoteGame } from '@/core/game';
import { useSettings } from '@/stores/settings';
import { localRef, useMutex } from '@tb-dev/vue';
import enUS_online from '@/locale/en-US/scenes/online.json';
import ptBR_online from '@/locale/pt-BR/scenes/online.json';
import enUS_hostGame from '@/locale/en-US/scenes/host-game.json';
import ptBR_hostGame from '@/locale/pt-BR/scenes/host-game.json';
import InputWorldName from '@/components/form/InputWorldName.vue';
import InputWorldSize from '@/components/form/InputWorldSize.vue';
import SliderBotDensity from '@/components/form/SliderBotDensity.vue';
import { isValidNullishPassword, isWorldOptions } from '@/lib/schema';
import type { WithPartialNullish, WritablePartial } from '@tb-dev/utils';
import InputWorldPassword from '@/components/form/InputWorldPassword.vue';
import TextareaWorldDescription from '@/components/form/TextareaWorldDescription.vue';
import SliderBotAdvancedStartRatio from '@/components/form/SliderBotAdvancedStartRatio.vue';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle, Checkbox, Label } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': toMerged(enUS_hostGame, enUS_online),
    'pt-BR': toMerged(ptBR_hostGame, ptBR_online),
  },
});

const router = useRouter();
const settings = useSettings();

const worldOptions = localRef<WritablePartial<WorldOptions>>(
  'host-remote-game:world',
  {
    name: null,
    size: __CONSTS__.continentSizeDefault,
    locale: settings.general.locale,
    allowCheats: false,
    botDensity: __CONSTS__.botDensityDefault,
    botAdvancedStartRatio: __CONSTS__.botAdvancedStartRatioDefault,
  } satisfies WithPartialNullish<WorldOptions, 'name'>,
);

const worldPassword = ref<Option<string>>();
const description = ref<Option<string>>();

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
      });
    }
  });
}
</script>

<template>
  <div class="card-layout">
    <Card class="md:min-w-150! md:max-w-1/2">
      <CardHeader>
        <CardTitle>{{ t('host-game') }}</CardTitle>
      </CardHeader>

      <CardContent>
        <InputWorldName v-model="worldOptions.name" :disabled="locked" />
        <InputWorldSize v-model="worldOptions.size" :disabled="locked" />
        <InputWorldPassword v-model="worldPassword" :disabled="locked" />
        <TextareaWorldDescription v-model="description" :disabled="locked" />
        <SliderBotDensity v-model="worldOptions" :disabled="locked" />
        <SliderBotAdvancedStartRatio v-model="worldOptions" :disabled="locked" />

        <div class="flex items-center justify-center py-1">
          <Label>
            <Checkbox v-model="worldOptions.allowCheats" :disabled="locked" />
            <span>{{ t('allow-cheats') }}</span>
          </Label>
        </div>
      </CardContent>

      <CardFooter class="w-full flex">
        <div class="w-full md:max-w-1/2 grid grid-cols-2 gap-2">
          <Button :disabled="locked || !canHost" @click="host">
            <span>{{ t('host') }}</span>
          </Button>

          <Button variant="secondary" :disabled="locked" @click="() => router.back()">
            <span>{{ t('cancel') }}</span>
          </Button>
        </div>
      </CardFooter>
    </Card>
  </div>
</template>
