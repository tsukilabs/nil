<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { toMerged } from 'es-toolkit';
import { useRouter } from 'vue-router';
import { hostRemoteGame } from '@/core/game';
import { useUserStore } from '@/stores/user';
import { useSettings } from '@/stores/settings';
import { localRef, useMutex } from '@tb-dev/vue';
import { WorldConfigImpl } from '@/core/model/world-config';
import enUS_online from '@/locale/en-US/scenes/online.json';
import ptBR_online from '@/locale/pt-BR/scenes/online.json';
import enUS_hostGame from '@/locale/en-US/scenes/host-game.json';
import ptBR_hostGame from '@/locale/pt-BR/scenes/host-game.json';
import { isValidNullishPassword, isWorldOptions } from '@/lib/schema';
import WorldNameInput from '@/components/host-game/WorldNameInput.vue';
import WorldSizeInput from '@/components/host-game/WorldSizeInput.vue';
import type { WithPartialNullish, WritablePartial } from '@tb-dev/utils';
import BotDensitySlider from '@/components/host-game/BotDensitySlider.vue';
import WorldPasswordInput from '@/components/host-game/WorldPasswordInput.vue';
import WorldDescriptionTextarea from '@/components/host-game/WorldDescriptionTextarea.vue';
import BotAdvancedStartRatioSlider from '@/components/host-game/BotAdvancedStartRatioSlider.vue';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle, Checkbox, Label } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': toMerged(enUS_hostGame, enUS_online),
    'pt-BR': toMerged(ptBR_hostGame, ptBR_online),
  },
});

const router = useRouter();
const settings = useSettings();

const userStore = useUserStore();
const { authorizationToken } = storeToRefs(userStore);

const worldOptions = localRef<WritablePartial<WorldOptions>>(
  'host-remote-game:world',
  {
    name: null,
    size: 100,
    locale: settings.locale,
    allowCheats: false,
    botDensity: WorldConfigImpl.DEFAULT_BOT_DENSITY,
    botAdvancedStartRatio: WorldConfigImpl.DEFAULT_BOT_ADVANCED_START_RATIO,
  } satisfies WithPartialNullish<WorldOptions, 'name'>,
);

const worldPassword = ref<Option<string>>();
const description = ref<Option<string>>();

const { locked, lock } = useMutex();
const isValidWorld = computed(() => isWorldOptions(worldOptions.value));
const canHost = computed(() => {
  return (
    isValidWorld.value &&
    Boolean(authorizationToken.value) &&
    isValidNullishPassword(worldPassword.value)
  );
});

async function host() {
  worldOptions.value.locale = settings.locale;
  await lock(async () => {
    if (isWorldOptions(worldOptions.value) && authorizationToken.value) {
      await hostRemoteGame({
        worldOptions: worldOptions.value,
        worldPassword: worldPassword.value,
        worldDescription: description.value,
        authorizationToken: authorizationToken.value,
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
        <WorldNameInput v-model="worldOptions.name" :disabled="locked" />
        <WorldSizeInput v-model="worldOptions.size" :disabled="locked" />
        <WorldPasswordInput v-model="worldPassword" :disabled="locked" />
        <WorldDescriptionTextarea v-model="description" :disabled="locked" />
        <BotDensitySlider v-model="worldOptions" :disabled="locked" />
        <BotAdvancedStartRatioSlider v-model="worldOptions" :disabled="locked" />

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
