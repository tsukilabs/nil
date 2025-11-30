<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { computed, nextTick } from 'vue';
import CostGrid from '@/components/resources/CostGrid.vue';
import type { ResourcesImpl } from '@/core/model/resources';
import { TableCell, TableRow } from '@tb-dev/vue-components';
import BuildCatalogRowAction from './BuildCatalogRowAction.vue';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import BuildingTitle from '@/components/infrastructure/BuildingTitle.vue';
import { usePrefectureSettings } from '@/settings/infrastructure/prefecture';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';
import type { BuildingImpl } from '@/core/model/infrastructure/building/abstract';
import type { PrefectureImpl } from '@/core/model/infrastructure/building/prefecture/prefecture';
import { useResolvedBuildingLevel } from '@/composables/infrastructure/useResolvedBuildingLevel';

const props = defineProps<{
  entry: PrefectureBuildCatalogEntry;
  building: BuildingImpl;
  prefecture: PrefectureImpl;
  scene: InfrastructureScene;
  loading: boolean;
  isPlayerTurn: boolean;
  playerResources: Option<ResourcesImpl>;
  onBuildOrder: (kind: PrefectureBuildOrderKind) => void;
  onToggle: () => void;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { player } = NIL.player.refs();

const settings = usePrefectureSettings();
const { hideMaxed, hideUnmet } = storeToRefs(settings);

const level = useResolvedBuildingLevel(() => props.building);

const canBuild = computed(() => {
  if (
    !props.loading &&
    player.value &&
    props.isPlayerTurn &&
    props.prefecture.enabled &&
    level.value.next <= level.value.max &&
    props.entry.kind === 'available'
  ) {
    return player.value.hasResources(props.entry.recipe.resources);
  }

  return false;
});

const canDemolish = computed(() => {
  return Boolean(
    !props.loading &&
      player.value &&
      props.isPlayerTurn &&
      props.prefecture.enabled &&
      level.value.previous >= level.value.min,
  );
});

const { sm } = useBreakpoints();

async function makeOrder(kind: PrefectureBuildOrderKind) {
  await nextTick();
  if (
    (kind === 'construction' && canBuild.value) ||
    (kind === 'demolition' && canDemolish.value)
  ) {
    props.onBuildOrder(kind);
  }
}
</script>

<template>
  <TableRow v-if="entry.kind === 'available'">
    <TableCell>
      <BuildingTitle :building="building.id" :level="building.level" :scene />
    </TableCell>

    <TableCell>
      <CostGrid
        v-if="sm"
        :resources="entry.recipe.resources"
        :maintenance="entry.recipe.maintenance"
        :workforce="entry.recipe.workforce"
        :limit="playerResources"
      />
      <template v-else>
        <div class="flex flex-col gap-2 py-2">
          <CostGrid
            :resources="entry.recipe.resources"
            :maintenance="entry.recipe.maintenance"
            :workforce="entry.recipe.workforce"
            :limit="playerResources"
          />
          <BuildCatalogRowAction
            :building
            :can-build
            :can-demolish
            :is-player-turn
            :loading
            @order="makeOrder"
            @toggle="onToggle"
          />
        </div>
      </template>
    </TableCell>

    <TableCell v-if="sm">
      <BuildCatalogRowAction
        :building
        :can-build
        :can-demolish
        :is-player-turn
        :loading
        @order="makeOrder"
        @toggle="onToggle"
      />
    </TableCell>
  </TableRow>

  <TableRow v-else-if="(entry.kind === 'maxed' && !hideMaxed) || (entry.kind === 'unmet' && !hideUnmet)">
    <TableCell>
      <BuildingTitle :building="building.id" :level="building.level" :scene />
    </TableCell>

    <TableCell :colspan="sm ? 2 : 1">
      <div class="text-muted-foreground flex w-full items-center justify-center text-sm">
        <span v-if="entry.kind === 'maxed'">
          {{ t('building-fully-constructed') }}
        </span>
        <span v-else-if="entry.kind === 'unmet'">
          {{ t('not-yet-available') }}
        </span>
      </div>
    </TableCell>
  </TableRow>

  <TableRow v-else class="hidden" />
</template>
