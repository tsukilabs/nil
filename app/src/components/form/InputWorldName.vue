<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Input } from '@ui/input';
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/host-game.json';
import ptBR from '@/locale/pt-BR/scenes/host-game.json';

defineProps<{
  disabled: boolean;
}>();

const name = defineModel<Option<string>>({ required: true });

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});
</script>

<template>
  <Label>
    <span>{{ t('world-name') }}</span>
    <Input
      v-model="name"
      type="text"
      :disabled
      :minlength="1"
      :maxlength="30"
    />
  </Label>
</template>
