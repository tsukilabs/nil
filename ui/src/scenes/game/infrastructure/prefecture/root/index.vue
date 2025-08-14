<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watch } from 'vue';
import BuildQueue from './BuildQueue.vue';
import BuildCatalog from './BuildCatalog.vue';
import { usePrefecture } from '@/composables/infrastructure/useBuilding';
import { useInfrastructure } from '@/composables/infrastructure/useInfrastructure';
import { usePrefectureBuildCatalog } from '@/composables/infrastructure/usePrefectureBuildCatalog';

const { coord, village } = NIL.village.refs();

const infrastructure = useInfrastructure();
const prefecture = usePrefecture();

const { catalog, loading, add, cancel, load, toggle } = usePrefectureBuildCatalog(coord);

await load();

watch(village, load);
</script>

<template>
  <div class="flex min-w-max w-full flex-col gap-4 xl:flex-row-reverse">
    <BuildQueue v-if="prefecture" :prefecture :loading @cancel="cancel" />
    <BuildCatalog
      v-if="infrastructure && catalog"
      :catalog
      :infrastructure
      :loading
      @build-order="add"
      @toggle="toggle"
    />
  </div>
</template>
