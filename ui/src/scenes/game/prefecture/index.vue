<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Catalog from './Catalog.vue';
import { asyncRef } from '@tb-dev/vue';
import BuildQueue from './BuildQueue.vue';
import { handleError } from '@/lib/error';
import { computed, ref, watch } from 'vue';
import { Card } from '@tb-dev/vue-components';
import {
  addPrefectureBuildOrder,
  cancelPrefectureBuildOrder,
  getPrefectureCatalog,
} from '@/commands';

const { coord, village } = NIL.village.refs();

const infrastructure = computed(() => village.value?.infrastructure);
const prefecture = computed(() => infrastructure.value?.prefecture);

const {
  state: catalog,
  execute: loadCatalog,
  isLoading: isLoadingCatalog,
} = asyncRef(null, async () => {
  return coord.value ? getPrefectureCatalog(coord.value) : null;
});

const isWaitingAddCmd = ref(false);
const isWaitingCancelCmd = ref(false);
const loading = computed(() => {
  return isLoadingCatalog.value || isWaitingAddCmd.value || isWaitingCancelCmd.value;
});

watch(coord, loadCatalog);

async function addBuildOrder(building: BuildingId, kind: PrefectureBuildOrderKind) {
  if (coord.value && !loading.value) {
    try {
      isWaitingAddCmd.value = true;
      await addPrefectureBuildOrder({ coord: coord.value, building, kind });
      await loadCatalog();
    } catch (err) {
      handleError(err);
    } finally {
      isWaitingAddCmd.value = false;
    }
  }
}

async function cancelBuildOrder() {
  if (coord.value && !loading.value) {
    try {
      isWaitingCancelCmd.value = true;
      await cancelPrefectureBuildOrder(coord.value);
      await loadCatalog();
    } catch (err) {
      handleError(err);
    } finally {
      isWaitingCancelCmd.value = false;
    }
  }
}
</script>

<template>
  <div class="game-layout">
    <Card
      v-if="infrastructure && prefecture"
      class="size-full"
      content-class="overflow-x-hidden overflow-y-auto px-2"
    >
      <template #title>
        <span>{{ `${$t('prefecture')} (${$t('level-x', [prefecture.level])})` }}</span>
      </template>

      <div class="flex w-full flex-col gap-4 xl:flex-row-reverse">
        <BuildQueue v-if="prefecture" :prefecture :loading @cancel="cancelBuildOrder" />
        <Catalog v-if="catalog" :catalog :infrastructure :loading @build-order="addBuildOrder" />
      </div>
    </Card>
  </div>
</template>
