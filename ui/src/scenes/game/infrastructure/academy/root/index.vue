<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watch } from 'vue';
import RecruitQueue from './RecruitQueue.vue';
import RecruitCatalog from './RecruitCatalog.vue';
import { useAcademy } from '@/composables/infrastructure/useBuilding';
import { useAcademyRecruitCatalog } from '@/composables/infrastructure/useAcademyRecruitCatalog';

const { coord, village } = NIL.village.refs();

const academy = useAcademy();
const { catalog, loading, add, cancel, load } = useAcademyRecruitCatalog(coord);

await load();

watch(village, load);
</script>

<template>
  <div class="flex w-full flex-col gap-4 xl:flex-row-reverse">
    <RecruitQueue v-if="academy" :academy :loading @cancel="cancel" />
    <RecruitCatalog
      v-if="academy && catalog"
      :academy
      :catalog
      :loading
      @recruit-order="add"
    />
  </div>
</template>
