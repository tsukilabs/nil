<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import RecruitCatalogRow from './RecruitCatalogRow.vue';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { usePlayerResources } from '@/composables/player/usePlayerResources';
import type { StableImpl } from '@/core/model/infrastructure/building/stable/stable';
import { Table, TableBody, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  stable: StableImpl;
  catalog: StableRecruitCatalog;
  loading: boolean;
  onRecruitOrder: (id: StableUnitId, chunks: number) => MaybePromise<void>;
}>();

const { t } = useI18n();

const isPlayerTurn = usePlayerTurn();
const playerResources = usePlayerResources();

const hasSomeAvailable = computed(() => {
  return Object.values(props.catalog).some((it) => it.kind === 'available');
});

const { sm } = useBreakpoints();
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow class="hover:bg-card">
        <TableHead>
          <span>{{ t('unit') }}</span>
        </TableHead>
        <TableHead v-if="hasSomeAvailable">
          <span>{{ t('cost') }}</span>
        </TableHead>
        <TableHead v-if="sm" :colspan="hasSomeAvailable ? 1 : 2">
          <span></span>
        </TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <RecruitCatalogRow
        unit="light-cavalry"
        :entry="catalog.lightCavalry"
        :stable
        :loading
        :is-player-turn
        :player-resources
        @recruit-order="(chunks) => onRecruitOrder('light-cavalry', chunks)"
      />
      <RecruitCatalogRow
        unit="heavy-cavalry"
        :entry="catalog.heavyCavalry"
        :stable
        :loading
        :is-player-turn
        :player-resources
        @recruit-order="(chunks) => onRecruitOrder('heavy-cavalry', chunks)"
      />
    </TableBody>
  </Table>
</template>
