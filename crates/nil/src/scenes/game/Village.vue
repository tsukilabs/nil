<script setup lang="ts">
import { useLocale } from '@/locale';
import { World } from '@/core/world';
import { VillageImpl } from '@/core/village';
import { computedAsync } from '@/composables/computed-async';

const { t } = useLocale();
const { currentVillage: current } = World.use();

const village = computedAsync(null, () => {
  return current.value ? VillageImpl.load(current.value) : null;
});
</script>

<template>
  <div class="size-full">
    <div v-if="village" class="size-full">
      <div>{{ `${village.name} (${village.coord.format()})` }}</div>
      <div class="flex flex-col gap-2">
        <div>{{ t('prefecture') }}</div>
        <div>{{ t('academy') }}</div>
        <div>{{ t('stable') }}</div>
        <div>{{ t('sawmill') }}</div>
        <div>{{ t('quarry') }}</div>
        <div>{{ t('iron-mine') }}</div>
        <div>{{ t('farm') }}</div>
        <div>{{ t('warehouse') }}</div>
        <div>{{ t('silo') }}</div>
        <div>{{ t('wall') }}</div>
      </div>
    </div>
  </div>
</template>
