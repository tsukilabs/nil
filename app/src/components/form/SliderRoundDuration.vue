<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import { Label, Slider, Switch } from '@tb-dev/vue-components';

defineProps<{
  disabled: boolean;
}>();

const duration = defineModel<RoundDuration>('duration', { required: true });
const enabled = defineModel<boolean>('enabled', { required: true });

const consts = __CONSTS__;

const sliderValue = computed({
  get: () => [duration.value],
  set: (value) => {
    duration.value = value.at(0) ?? consts.roundDurationDefault;
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
  <Label for="slider-round-duration">
    <span>{{ t('round-duration') }}</span>
    <div class="flex flex-row items-center gap-1">
      <Switch v-model="enabled" />
      <Slider
        id="slider-round-duration"
        v-model:model-value="sliderValue"
        :disabled="disabled || !enabled"
        :min="consts.roundDurationMin"
        :max="consts.roundDurationMax"
        :step="1"
      />
      <span>{{ duration }}</span>
    </div>
  </Label>
</template>
