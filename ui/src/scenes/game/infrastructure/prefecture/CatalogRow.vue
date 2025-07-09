<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, nextTick } from 'vue';
import CatalogCellBuilding from './CatalogCellBuilding.vue';
import { useBuildingLevel } from '@/composables/prefecture';
import type { BuildingImpl } from '@/core/model/buildings/abstract';
import { Button, TableCell, TableRow } from '@tb-dev/vue-components';
import type { PrefectureImpl } from '@/core/model/buildings/prefecture';

const props = defineProps<{
  entry: PrefectureBuildCatalogEntry;
  building: BuildingImpl;
  prefecture: PrefectureImpl;
  scene: GameScene;
  loading: boolean;
  onBuildOrder: (kind: PrefectureBuildOrderKind) => void;
}>();

const { t } = useI18n();

const { player } = NIL.player.refs();
const { isPlayerTurn } = NIL.round.refs();

const level = useBuildingLevel(
  () => props.prefecture,
  () => props.building
);

const canBuild = computed(() => {
  if (
    !props.loading &&
    player.value &&
    isPlayerTurn.value &&
    props.prefecture.enabled &&
    level.value.next <= level.value.max &&
    props.entry.kind === 'available'
  ) {
    return player.value.hasResources(props.entry.recipe.resources);
  }

  return false;
});

const canDemolish = computed(() => {
  return (
    !props.loading &&
    player.value &&
    isPlayerTurn.value &&
    props.prefecture.enabled &&
    level.value.previous >= level.value.min
  );
});

async function makeOrder(kind: PrefectureBuildOrderKind) {
  await nextTick();
  if ((kind === 'construction' && canBuild.value) || (kind === 'demolition' && canDemolish.value)) {
    props.onBuildOrder(kind);
  }
}
</script>

<template>
  <TableRow v-if="entry.kind === 'available'">
    <TableCell class="min-w-24">
      <CatalogCellBuilding :building :scene />
    </TableCell>
    <TableCell>
      <div class="grid grid-cols-3 items-center justify-start gap-4">
        <Wood :amount="entry.recipe.resources.wood" />
        <Stone :amount="entry.recipe.resources.stone" />
        <Iron :amount="entry.recipe.resources.iron" />
      </div>
    </TableCell>
    <TableCell>
      <div class="flex items-center justify-start">
        <Food :amount="entry.recipe.maintenance" />
      </div>
    </TableCell>
    <TableCell>
      <div class="flex items-center justify-start">
        <Workforce :amount="entry.recipe.workforce" />
      </div>
    </TableCell>
    <TableCell class="min-w-30">
      <div class="grid max-w-fit grid-cols-2 items-center justify-start gap-4">
        <Button
          size="sm"
          :disabled="!canBuild"
          class="max-w-24"
          @click="() => makeOrder('construction')"
        >
          <span>{{ t('build') }}</span>
        </Button>
        <Button
          variant="destructive"
          size="sm"
          :disabled="!canDemolish"
          class="max-w-24"
          @click="() => makeOrder('demolition')"
        >
          <span>{{ t('demolish') }}</span>
        </Button>
      </div>
    </TableCell>
  </TableRow>

  <TableRow v-else-if="entry.kind === 'maxed'">
    <TableCell>
      <CatalogCellBuilding :building :scene />
    </TableCell>
    <TableCell colspan="4" class="w-full">
      <div class="text-muted-foreground flex w-full items-center justify-center text-sm">
        <span>{{ t('building-fully-constructed') }}</span>
      </div>
    </TableCell>
  </TableRow>
</template>
