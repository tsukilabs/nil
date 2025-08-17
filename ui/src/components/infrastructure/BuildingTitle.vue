<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

defineProps<{
  building: BuildingId;
  level: BuildingLevel;
  scene?: InfrastructureScene;
}>();

const { t } = useI18n();
</script>

<template>
  <div class="flex flex-col gap-1">
    <RouterLink v-if="scene" :to="{ name: scene }" class="md:whitespace-nowrap" @click.stop>
      {{ t(building) }}
    </RouterLink>
    <span v-else class="md:whitespace-nowrap">
      {{ t(building) }}
    </span>

    <span class="text-muted-foreground text-xs">
      {{ t('level-x', [level]) }}
    </span>
  </div>
</template>
