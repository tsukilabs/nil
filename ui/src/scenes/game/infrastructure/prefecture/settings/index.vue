<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { renameCity } from '@/commands';
import { Button, Checkbox, Input, Label } from '@tb-dev/vue-components';
import { usePrefectureSettings } from '@/settings/infrastructure/prefecture';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { coord, city } = NIL.city.refs();

const settings = usePrefectureSettings();
const { hideMaxed, hideUnmet } = storeToRefs(settings);

const cityName = ref(city.value?.name);

function rename() {
  if (coord.value && cityName.value) {
    renameCity(coord.value, cityName.value).err();
  }
}
</script>

<template>
  <div class="size-full px-4">
    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-2">
        <Label>
          <Checkbox v-model="hideMaxed" />
          <span>{{ t('hide-fully-constructed') }}</span>
        </Label>

        <Label>
          <Checkbox v-model="hideUnmet" />
          <span>{{ t('hide-unavailable-buildings') }}</span>
        </Label>
      </div>

      <Label class="max-w-96 py-1">
        <span class="text-muted-foreground">{{ t('rename-city') }}</span>
        <div class="flex items-center gap-2">
          <Input
            v-model.trim="cityName"
            type="text"
            minlength="1"
            maxlength="50"
            spellcheck="false"
          />
          <Button size="sm" :disabled="!city" @click="rename">
            <span>{{ t('rename') }}</span>
          </Button>
        </div>
      </Label>
    </div>
  </div>
</template>
