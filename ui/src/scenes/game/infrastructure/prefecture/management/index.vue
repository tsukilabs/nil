<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { renameVillage } from '@/commands';
import { Button, Input, Label } from '@tb-dev/vue-components';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { coord, village } = NIL.village.refs();

const villageName = ref(village.value?.name);

function rename() {
  if (coord.value && villageName.value) {
    renameVillage(coord.value, villageName.value).err();
  }
}
</script>

<template>
  <div class="size-full px-4">
    <div class="">
      <Label class="max-w-80">
        <span class="text-muted-foreground">{{ t('rename-village') }}</span>
        <div class="flex items-center gap-2">
          <Input
            v-model="villageName"
            type="text"
            minlength="1"
            maxlength="50"
            spellcheck="false"
          />
          <Button size="sm" :disabled="!village" @click="rename">
            <span>{{ t('rename') }}</span>
          </Button>
        </div>
      </Label>
    </div>
  </div>
</template>
