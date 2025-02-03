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
        <div>{{ t('misc.prefecture') }}</div>
        <div>{{ t('misc.academy') }}</div>
        <div>{{ t('misc.stable') }}</div>
        <div>{{ t('misc.sawmill') }}</div>
        <div>{{ t('misc.quarry') }}</div>
        <div>{{ t('misc.iron-mine') }}</div>
        <div>{{ t('misc.farm') }}</div>
        <div>{{ t('misc.warehouse') }}</div>
        <div>{{ t('misc.wall') }}</div>
      </div>
    </div>
  </div>
</template>
