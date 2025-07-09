<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { BuildingImpl } from '@/core/model/buildings/abstract';

defineProps<{
  building: BuildingImpl;
  scene: GameScene;
}>();

const { t } = useI18n();
</script>

<template>
  <div class="flex flex-col gap-1">
    <RouterLink :to="{ name: scene }">{{ t(building.id) }}</RouterLink>
    <span class="text-muted-foreground text-xs">
      {{ t('level-x', [building.level]) }}
    </span>
  </div>
</template>
