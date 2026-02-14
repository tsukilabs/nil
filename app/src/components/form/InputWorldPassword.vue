<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { Input, Label } from '@tb-dev/vue-components';

defineProps<{
  disabled: boolean;
  onEnter?: () => void;
}>();

const password = defineModel<Option<string>>({ required: true });

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});
</script>

<template>
  <Label>
    <span>{{ t('world-password') }}</span>
    <Input
      v-model="password"
      type="password"
      :disabled
      :minlength="3"
      :maxlength="50"
      @keydown.enter="onEnter"
    />
  </Label>
</template>
