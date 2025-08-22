<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { ResourcesImpl } from '@/core/model/resources';
import CostGrid from '@/components/resources/CostGrid.vue';
import { TableCell, TableRow } from '@tb-dev/vue-components';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { useStableSettings } from '@/settings/infrastructure/stable';
import type { StableImpl } from '@/core/model/infrastructure/building/stable/stable';
import { useRecruitCatalogEntry } from '@/composables/infrastructure/useRecruitCatalogEntry';
import RecruitCatalogRowAction from '@/components/infrastructure/RecruitCatalogRowAction.vue';

const props = defineProps<{
  entry: StableRecruitCatalogEntry;
  unit: StableUnitId;
  stable: StableImpl;
  loading: boolean;
  isPlayerTurn: boolean;
  playerResources: Option<ResourcesImpl>;
  onRecruitOrder: (chunks: number) => void;
}>();

const { t } = useI18n();

const { player } = NIL.player.refs();

const settings = useStableSettings();
const { hideUnmet } = storeToRefs(settings);

const {
  chunks,
  resources,
  maintenance,
  workforce,
} = useRecruitCatalogEntry(() => props.entry);

const canRecruit = computed(() => {
  if (
    !props.loading &&
    player.value &&
    props.isPlayerTurn &&
    props.stable.enabled &&
    props.entry.kind === 'available'
  ) {
    return player.value.hasResources(resources.value);
  }

  return false;
});

const { sm } = useBreakpoints();
</script>

<template>
  <TableRow v-if="entry.kind === 'available'">
    <TableCell class="min-w-24">
      <span>{{ t(unit) }}</span>
    </TableCell>

    <TableCell>
      <CostGrid
        v-if="sm"
        :resources
        :maintenance
        :workforce
        :limit="playerResources"
      />
      <template v-else>
        <div class="flex flex-col gap-2 py-2">
          <CostGrid
            :resources
            :maintenance
            :workforce
            :limit="playerResources"
          />
          <RecruitCatalogRowAction
            v-model="chunks"
            :can-recruit
            :loading
            @recruit-order="onRecruitOrder"
          />
        </div>
      </template>
    </TableCell>

    <TableCell v-if="sm" class="min-w-30">
      <RecruitCatalogRowAction
        v-model="chunks"
        :can-recruit
        :loading
        @recruit-order="onRecruitOrder"
      />
    </TableCell>
  </TableRow>

  <TableRow v-else-if="entry.kind === 'unmet' && !hideUnmet">
    <TableCell class="min-w-24">
      <span>{{ t(unit) }}</span>
    </TableCell>
    <TableCell :colspan="sm ? 2 : 1" class="w-full">
      <div class="text-muted-foreground flex w-full items-center justify-center text-sm">
        <span>{{ t('not-yet-available') }}</span>
      </div>
    </TableCell>
  </TableRow>

  <TableRow v-else class="hidden" />
</template>
