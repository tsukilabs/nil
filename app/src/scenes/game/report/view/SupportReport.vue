<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { CoordImpl } from '@/core/model/continent/coord';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import SupportReportPersonnel from './SupportReportPersonnel.vue';
import type { SupportReportImpl } from '@/core/model/report/support-report';
import { useCityOwnerSceneLink } from '@/composables/city/useCityOwnerSceneLink';
import { useCityProfileSceneLink } from '@/composables/city/useCityProfileSceneLink';
import { Table, TableBody, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  report: SupportReportImpl;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const toRulerProfile = useCityOwnerSceneLink(() => props.report.sender);
const toOriginProfile = useCityProfileSceneLink(() => props.report.originCity);
const toDestinationProfile = useCityProfileSceneLink(() => props.report.destinationCity);
</script>

<template>
  <div class="w-full min-w-max flex flex-col gap-4">
    <Table class="max-w-[800px]">
      <TableBody>
        <TableRow>
          <TableHead>{{ t('player') }}</TableHead>
          <TableCell>
            <RouterLink v-if="toRulerProfile" :to="toRulerProfile">
              {{ report.sender.id }}
            </RouterLink>
          </TableCell>
        </TableRow>

        <TableRow>
          <TableHead>{{ t('origin') }}</TableHead>
          <TableCell>
            <RouterLink v-if="toOriginProfile" :to="toOriginProfile">
              {{ `${report.originCity.name} (${CoordImpl.format(report.originCity.coord)})` }}
            </RouterLink>
          </TableCell>
        </TableRow>

        <TableRow>
          <TableHead>{{ t('destination') }}</TableHead>
          <TableCell>
            <RouterLink v-if="toDestinationProfile" :to="toDestinationProfile">
              {{ `${report.destinationCity.name} (${CoordImpl.format(report.destinationCity.coord)})` }}
            </RouterLink>
          </TableCell>
        </TableRow>

        <TableRow class="hover:bg-card">
          <TableCell colspan="2">
            <SupportReportPersonnel :personnel="report.personnel" />
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
