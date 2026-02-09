<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { formatPercent } from '@/lib/intl';
import { hostLocalGame } from '@/core/game';
import { useSettings } from '@/stores/settings';
import { localRef, useMutex } from '@tb-dev/vue';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import { WorldConfigImpl } from '@/core/model/world-config';
import { isPlayerOptions, isWorldOptions } from '@/lib/schema';
import type { WithPartialNullish, WritablePartial } from '@tb-dev/utils';
import { useBotDensitySlider } from '@/composables/world/useBotDensitySlider';
import { useBotAdvancedStartRatioSlider } from '@/composables/world/useBotAdvancedStartRatioSlider';
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
  Slider,
} from '@tb-dev/vue-components';

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

const botDensity = useBotDensitySlider(worldOptions);
const botAdvancedStartRatio = useBotAdvancedStartRatioSlider(worldOptions);

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

        <Label>
          <span>{{ t('bot-density') }}</span>
          <div>
            <Slider
              v-model:model-value="botDensity"
              :disabled="locked"
              :min="0"
              :max="3"
              :step="0.01"
            />
            <span>{{ formatPercent(botDensity[0]) }}</span>
          </div>
        </Label>

        <Label>
          <span>{{ t('advanced-bots-ratio') }}</span>
          <div>
            <Slider
              v-model:model-value="botAdvancedStartRatio"
              :disabled="locked"
              :min="0"
              :max="1"
              :step="0.01"
            />
            <span>{{ formatPercent(botAdvancedStartRatio[0]) }}</span>
          </div>
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
