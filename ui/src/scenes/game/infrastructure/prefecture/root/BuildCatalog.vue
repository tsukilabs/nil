<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { MaybePromise } from '@tb-dev/utils';
import BuildCatalogRow from './BuildCatalogRow.vue';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';
import type { InfrastructureImpl } from '@/core/model/infrastructure';
import { usePlayerResources } from '@/composables/player/usePlayerResources';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';
import { Table, TableBody, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  catalog: PrefectureBuildCatalog;
  infrastructure: InfrastructureImpl;
  loading: boolean;
  onBuildOrder: (id: BuildingId, kind: PrefectureBuildOrderKind) => MaybePromise<void>;
  onToggle: (id: BuildingId, enabled: boolean) => MaybePromise<void>;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const isPlayerTurn = usePlayerTurn();
const playerResources = usePlayerResources();

const hasSomeAvailable = computed(() => {
  return Object.values(props.catalog).some((it) => it.kind === 'available');
});
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow class="hover:bg-card">
        <TableHead>
          <span>{{ t('building') }}</span>
        </TableHead>
        <TableHead v-if="hasSomeAvailable">
          <span>{{ t('cost') }}</span>
        </TableHead>
        <TableHead v-if="hasSomeAvailable">
          <span>{{ t('maintenance') }}</span>
        </TableHead>
        <TableHead v-if="hasSomeAvailable">
          <span>{{ t('workforce') }}</span>
        </TableHead>
        <TableHead :colspan="hasSomeAvailable ? 1 : 4">
          <span></span>
        </TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <BuildCatalogRow
        :entry="catalog.prefecture"
        :building="infrastructure.prefecture"
        :prefecture="infrastructure.prefecture"
        scene="prefecture"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('prefecture', kind)"
        @toggle="() => onToggle('prefecture', !infrastructure.prefecture.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.academy"
        :building="infrastructure.academy"
        :prefecture="infrastructure.prefecture"
        scene="academy"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('academy', kind)"
        @toggle="() => onToggle('academy', !infrastructure.academy.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.stable"
        :building="infrastructure.stable"
        :prefecture="infrastructure.prefecture"
        scene="stable"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('stable', kind)"
        @toggle="() => onToggle('stable', !infrastructure.stable.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.sawmill"
        :building="infrastructure.sawmill"
        :prefecture="infrastructure.prefecture"
        scene="sawmill"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('sawmill', kind)"
        @toggle="() => onToggle('sawmill', !infrastructure.sawmill.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.quarry"
        :building="infrastructure.quarry"
        :prefecture="infrastructure.prefecture"
        scene="quarry"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('quarry', kind)"
        @toggle="() => onToggle('quarry', !infrastructure.quarry.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.ironMine"
        :building="infrastructure.ironMine"
        :prefecture="infrastructure.prefecture"
        scene="iron-mine"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('iron-mine', kind)"
        @toggle="() => onToggle('iron-mine', !infrastructure.ironMine.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.farm"
        :building="infrastructure.farm"
        :prefecture="infrastructure.prefecture"
        scene="farm"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('farm', kind)"
        @toggle="() => onToggle('farm', !infrastructure.farm.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.warehouse"
        :building="infrastructure.warehouse"
        :prefecture="infrastructure.prefecture"
        scene="warehouse"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('warehouse', kind)"
        @toggle="() => onToggle('warehouse', !infrastructure.warehouse.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.silo"
        :building="infrastructure.silo"
        :prefecture="infrastructure.prefecture"
        scene="silo"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('silo', kind)"
        @toggle="() => onToggle('silo', !infrastructure.silo.enabled)"
      />
      <BuildCatalogRow
        :entry="catalog.wall"
        :building="infrastructure.wall"
        :prefecture="infrastructure.prefecture"
        scene="wall"
        :loading
        :is-player-turn
        :player-resources
        @build-order="(kind) => onBuildOrder('wall', kind)"
        @toggle="() => onToggle('wall', !infrastructure.wall.enabled)"
      />
    </TableBody>
  </Table>
</template>
