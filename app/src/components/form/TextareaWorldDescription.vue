<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { Label, Textarea } from '@tb-dev/vue-components';

defineProps<{
  disabled: boolean;
}>();

const description = defineModel<Option<string>>({ required: true });

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});
</script>

<template>
  <Label>
    <span>{{ t('world-description') }}</span>
    <Textarea
      v-model="description"
      type="text"
      spellcheck="false"
      autocapitalize="off"
      autocomplete="off"
      :disabled
      :maxlength="300"
      class="max-h-24 resize-none"
    />
  </Label>
</template>
