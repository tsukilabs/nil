<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';
import { Slider } from '@ui/slider';
import type { WritablePartial } from '@tb-dev/utils';
import type { WorldOptions } from '@/types/core/world';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';

defineProps<{
  disabled: boolean;
}>();

const worldOptions = defineModel<WritablePartial<WorldOptions>>({ required: true });

const consts = __CONSTS__;

const sliderValue = computed({
  get: () => [worldOptions.value.unitSpeed ?? consts.worldUnitSpeedDefault],
  set: (value) => {
    worldOptions.value.unitSpeed = value.at(0) ?? consts.worldUnitSpeedDefault;
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
    <span>{{ t('world-unit-speed') }}</span>
    <div>
      <Slider
        v-model:model-value="sliderValue"
        :disabled
        :min="consts.worldUnitSpeedMin"
        :max="consts.worldUnitSpeedMax"
        :step="0.1"
      />
      <span>{{ `${intl.format(sliderValue[0])}x` }}</span>
    </div>
  </Label>
</template>
