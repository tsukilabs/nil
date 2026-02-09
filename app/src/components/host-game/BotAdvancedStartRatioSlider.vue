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
const defaultValue = WorldConfigImpl.DEFAULT_BOT_ADVANCED_START_RATIO;

const botAdvancedStartRatio = computed({
  get: () => [worldOptions.value.botAdvancedStartRatio ?? defaultValue],
  set: (value) => {
    worldOptions.value.botAdvancedStartRatio = value.at(0) ?? defaultValue;
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
    <span>{{ t('advanced-bots-ratio') }}</span>
    <div>
      <Slider
        v-model:model-value="botAdvancedStartRatio"
        :disabled
        :min="consts.botAdvancedStartRatioMin"
        :max="consts.botAdvancedStartRatioMax"
        :step="0.01"
      />
      <span>{{ formatPercent(botAdvancedStartRatio[0]) }}</span>
    </div>
  </Label>
</template>
