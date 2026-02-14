<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { formatPercent } from '@/lib/intl';
import type { WritablePartial } from '@tb-dev/utils';
import { Label, Slider } from '@tb-dev/vue-components';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import { WorldConfigImpl } from '@/core/model/world-config';

defineProps<{
  disabled: boolean;
}>();

const worldOptions = defineModel<WritablePartial<WorldOptions>>({ required: true });

const consts = __CONSTS__;
const defaultValue = WorldConfigImpl.DEFAULT_BOT_DENSITY;

const botDensity = computed({
  get: () => [worldOptions.value.botDensity ?? defaultValue],
  set: (value) => {
    worldOptions.value.botDensity = value.at(0) ?? defaultValue;
  },
});

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});
</script>

<template>
  <Label>
    <span>{{ t('bot-density') }}</span>
    <div>
      <Slider
        v-model:model-value="botDensity"
        :disabled
        :min="consts.botDensityMin"
        :max="consts.botDensityMax"
        :step="0.01"
      />
      <span>{{ formatPercent(botDensity[0]) }}</span>
    </div>
  </Label>
</template>
