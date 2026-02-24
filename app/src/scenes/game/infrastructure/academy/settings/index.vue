<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useSettings } from '@/stores/settings';
import { Checkbox, Label } from '@tb-dev/vue-components';
import enUS from '@/locale/en-US/scenes/game/infrastructure/infrastructure.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/infrastructure.json';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const settings = useSettings();
</script>

<template>
  <div class="size-full px-4">
    <div class="flex flex-col">
      <Label>
        <Checkbox v-model="settings.academy.hideUnmet" />
        <span>{{ t('hide-unavailable-units') }}</span>
      </Label>
    </div>
  </div>
</template>
