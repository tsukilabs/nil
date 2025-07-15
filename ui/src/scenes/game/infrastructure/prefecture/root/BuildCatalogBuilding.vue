<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { BuildingImpl } from '@/core/model/building/abstract';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';

defineProps<{
  building: BuildingImpl;
  scene: GameScene;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});
</script>

<template>
  <div class="flex flex-col gap-1">
    <RouterLink :to="{ name: scene }">{{ t(building.id) }}</RouterLink>
    <span class="text-muted-foreground text-xs">
      {{ t('level-x', [building.level]) }}
    </span>
  </div>
</template>
