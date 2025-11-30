<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { Checkbox, Label } from '@tb-dev/vue-components';
import { useStableSettings } from '@/settings/infrastructure/stable';
import enUS from '@/locale/en-US/scenes/game/infrastructure/infrastructure.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/infrastructure.json';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const settings = useStableSettings();
const { hideUnmet } = storeToRefs(settings);
</script>

<template>
  <div class="size-full px-4">
    <div class="flex flex-col gap-4">
      <Label>
        <Checkbox v-model="hideUnmet" />
        <span>{{ t('hide-unavailable-units') }}</span>
      </Label>
    </div>
  </div>
</template>
