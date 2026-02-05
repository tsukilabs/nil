<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watch } from 'vue';
import RecruitQueue from './RecruitQueue.vue';
import RecruitCatalog from './RecruitCatalog.vue';
import { useWorkshop } from '@/composables/infrastructure/useBuilding';
import { useWorkshopRecruitCatalog } from '@/composables/infrastructure/useWorkshopRecruitCatalog';

const { coord, city } = NIL.city.refs();

const workshop = useWorkshop();
const { catalog, loading, add, cancel, load } = useWorkshopRecruitCatalog(coord);

await load();

watch(city, load);
</script>

<template>
  <div class="flex w-full min-w-max flex-col gap-4 xl:flex-row-reverse">
    <RecruitQueue v-if="workshop" :workshop :loading @cancel="cancel" />
    <RecruitCatalog
      v-if="workshop && catalog"
      :workshop
      :catalog
      :loading
      @recruit-order="add"
    />
  </div>
</template>
