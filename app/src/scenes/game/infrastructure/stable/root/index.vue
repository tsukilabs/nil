<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watch } from 'vue';
import RecruitQueue from './RecruitQueue.vue';
import RecruitCatalog from './RecruitCatalog.vue';
import { useStable } from '@/composables/infrastructure/useBuilding';
import { useStableRecruitCatalog } from '@/composables/infrastructure/useStableRecruitCatalog';

const { coord, city } = NIL.city.refs();

const stable = useStable();
const { catalog, loading, add, cancel, load } = useStableRecruitCatalog(coord);

await load();

watch(city, load);
</script>

<template>
  <div class="flex w-full min-w-max flex-col gap-4 xl:flex-row-reverse">
    <RecruitQueue v-if="stable" :stable :loading @cancel="cancel" />
    <RecruitCatalog
      v-if="stable && catalog"
      :stable
      :catalog
      :loading
      @recruit-order="add"
    />
  </div>
</template>
