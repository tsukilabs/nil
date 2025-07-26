<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watch } from 'vue';
import { useI18n } from 'vue-i18n';
import RecruitQueue from './RecruitQueue.vue';
import RecruitCatalog from './RecruitCatalog.vue';
import { useStable } from '@/composables/infrastructure/useBuilding';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';
import { useStableRecruitCatalog } from '@/composables/infrastructure/useStableRecruitCatalog';

const { t } = useI18n();

const { coord, village } = NIL.village.refs();

const stable = useStable();
const { catalog, loading, add, cancel, load } = useStableRecruitCatalog(coord);

await load();

watch(village, load);
</script>

<template>
  <div class="game-layout">
    <Card v-if="stable" class="size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span>{{ `${t('stable')} (${t('level-x', [stable.level])})` }}</span>
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="overflow-x-hidden overflow-y-auto px-2 py-0">
        <div class="flex w-full flex-col gap-4 xl:flex-row-reverse">
          <RecruitQueue v-if="stable" :stable :loading @cancel="cancel" />
          <RecruitCatalog
            v-if="stable && catalog"
            :stable
            :catalog
            :loading
            @recruit-order="add"
          />
        </div>
      </CardContent>
    </Card>
  </div>
</template>
