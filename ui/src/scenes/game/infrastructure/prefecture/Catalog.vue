<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import CatalogRow from './CatalogRow.vue';
import type { MaybePromise } from '@tb-dev/utils';
import { Table, TableHead, TableRow } from '@tb-dev/vue-components';
import type { InfrastructureImpl } from '@/core/model/infrastructure';

const props = defineProps<{
  catalog: PrefectureBuildCatalog;
  infrastructure: InfrastructureImpl;
  loading: boolean;
  onBuildOrder: (id: BuildingId, kind: PrefectureBuildOrderKind) => MaybePromise<void>;
}>();

const hasSomeAvailable = computed(() => {
  return Object.values(props.catalog).some((it) => it.kind === 'available');
});
</script>

<template>
  <Table>
    <template #header>
      <TableRow class="bg-background hover:bg-background">
        <TableHead>
          <span>{{ $t('building') }}</span>
        </TableHead>
        <TableHead v-if="hasSomeAvailable">
          <span>{{ $t('cost') }}</span>
        </TableHead>
        <TableHead v-if="hasSomeAvailable">
          <span>{{ $t('maintenance') }}</span>
        </TableHead>
        <TableHead v-if="hasSomeAvailable">
          <span>{{ $t('workforce') }}</span>
        </TableHead>
        <TableHead :colspan="hasSomeAvailable ? 1 : 4">
          <span></span>
        </TableHead>
      </TableRow>
    </template>

    <CatalogRow
      :entry="catalog.prefecture"
      :building="infrastructure.prefecture"
      :prefecture="infrastructure.prefecture"
      scene="prefecture"
      :loading
      @build-order="(kind) => onBuildOrder('prefecture', kind)"
    />
    <CatalogRow
      :entry="catalog.academy"
      :building="infrastructure.academy"
      :prefecture="infrastructure.prefecture"
      scene="academy"
      :loading
      @build-order="(kind) => onBuildOrder('academy', kind)"
    />
    <CatalogRow
      :entry="catalog.stable"
      :building="infrastructure.stable"
      :prefecture="infrastructure.prefecture"
      scene="stable"
      :loading
      @build-order="(kind) => onBuildOrder('stable', kind)"
    />
    <CatalogRow
      :entry="catalog.sawmill"
      :building="infrastructure.sawmill"
      :prefecture="infrastructure.prefecture"
      scene="sawmill"
      :loading
      @build-order="(kind) => onBuildOrder('sawmill', kind)"
    />
    <CatalogRow
      :entry="catalog.quarry"
      :building="infrastructure.quarry"
      :prefecture="infrastructure.prefecture"
      scene="quarry"
      :loading
      @build-order="(kind) => onBuildOrder('quarry', kind)"
    />
    <CatalogRow
      :entry="catalog.ironMine"
      :building="infrastructure.ironMine"
      :prefecture="infrastructure.prefecture"
      scene="iron-mine"
      :loading
      @build-order="(kind) => onBuildOrder('iron-mine', kind)"
    />
    <CatalogRow
      :entry="catalog.farm"
      :building="infrastructure.farm"
      :prefecture="infrastructure.prefecture"
      scene="farm"
      :loading
      @build-order="(kind) => onBuildOrder('farm', kind)"
    />
    <CatalogRow
      :entry="catalog.warehouse"
      :building="infrastructure.warehouse"
      :prefecture="infrastructure.prefecture"
      scene="warehouse"
      :loading
      @build-order="(kind) => onBuildOrder('warehouse', kind)"
    />
    <CatalogRow
      :entry="catalog.silo"
      :building="infrastructure.silo"
      :prefecture="infrastructure.prefecture"
      scene="silo"
      :loading
      @build-order="(kind) => onBuildOrder('silo', kind)"
    />
    <CatalogRow
      :entry="catalog.wall"
      :building="infrastructure.wall"
      :prefecture="infrastructure.prefecture"
      scene="wall"
      :loading
      @build-order="(kind) => onBuildOrder('wall', kind)"
    />
  </Table>
</template>
