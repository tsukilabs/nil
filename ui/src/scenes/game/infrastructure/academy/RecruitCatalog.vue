<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import RecruitCatalogRow from './RecruitCatalogRow.vue';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';
import { usePlayerResources } from '@/composables/player/usePlayerResources';
import type { AcademyImpl } from '@/core/model/infrastructure/building/academy';
import { Table, TableBody, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  academy: AcademyImpl;
  catalog: AcademyRecruitCatalog;
  loading: boolean;
  onRecruitOrder: (id: AcademyUnitId, chunks: number) => MaybePromise<void>;
}>();

const { t } = useI18n();

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
          <span>{{ t('unit') }}</span>
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
      <RecruitCatalogRow
        unit="pikeman"
        :entry="catalog.pikeman"
        :academy
        :loading
        :is-player-turn
        :player-resources
        @recruit-order="(chunks) => onRecruitOrder('pikeman', chunks)"
      />
      <RecruitCatalogRow
        unit="swordsman"
        :entry="catalog.swordsman"
        :academy
        :loading
        :is-player-turn
        :player-resources
        @recruit-order="(chunks) => onRecruitOrder('swordsman', chunks)"
      />
      <RecruitCatalogRow
        unit="axeman"
        :entry="catalog.axeman"
        :academy
        :loading
        :is-player-turn
        :player-resources
        @recruit-order="(chunks) => onRecruitOrder('axeman', chunks)"
      />
      <RecruitCatalogRow
        unit="archer"
        :entry="catalog.archer"
        :academy
        :loading
        :is-player-turn
        :player-resources
        @recruit-order="(chunks) => onRecruitOrder('archer', chunks)"
      />
    </TableBody>
  </Table>
</template>
