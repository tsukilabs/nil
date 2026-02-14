<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';
import {
  Label,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@tb-dev/vue-components';

defineProps<{
  disabled: boolean;
}>();

const size = defineModel<Option<number>>({ required: true });

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const consts = __CONSTS__;
</script>

<template>
  <Label>
    <span>{{ t('world-size') }}</span>
    <NumberField
      v-model="size"
      :disabled
      :min="consts.continentSizeMin"
      :max="consts.continentSizeMax"
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
</template>
