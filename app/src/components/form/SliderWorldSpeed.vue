<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';
import { Slider } from '@ui/slider';
import type { WritablePartial } from '@tb-dev/utils';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';

defineProps<{
  disabled: boolean;
}>();

const worldOptions = defineModel<WritablePartial<WorldOptions>>({ required: true });

const consts = __CONSTS__;

const sliderValue = computed({
  get: () => [worldOptions.value.speed ?? consts.worldSpeedDefault],
  set: (value) => {
    worldOptions.value.speed = value.at(0) ?? consts.worldSpeedDefault;
  },
});

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const intl = new Intl.NumberFormat(undefined, {
  style: 'decimal',
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
  useGrouping: false,
});
</script>

<template>
  <Label>
    <span>{{ t('world-speed') }}</span>
    <div>
      <Slider
        v-model:model-value="sliderValue"
        :disabled
        :min="consts.worldSpeedMin"
        :max="consts.worldSpeedMax"
        :step="0.1"
      />
      <span>{{ `${intl.format(sliderValue[0])}x` }}</span>
    </div>
  </Label>
</template>
