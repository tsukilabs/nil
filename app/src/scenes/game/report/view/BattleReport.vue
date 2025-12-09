<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { CoordImpl } from '@/core/model/continent/coord';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { useBattleLosses } from '@/composables/battle/useBattleLosses';
import type { BattleReportImpl } from '@/core/model/report/battle-report';
import { useCityOwnerSceneLink } from '@/composables/city/useCityOwnerSceneLink';
import { useCityProfileSceneLink } from '@/composables/city/useCityProfileSceneLink';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  report: BattleReportImpl;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const losses = useBattleLosses(() => props.report.result);

const toAttackerProfile = useCityOwnerSceneLink(() => props.report.attacker);
const toDefenderProfile = useCityOwnerSceneLink(() => props.report.defender);
const toOriginProfile = useCityProfileSceneLink(() => props.report.originCity);
const toDestinationProfile = useCityProfileSceneLink(() => props.report.destinationCity);
</script>

<template>
  <div class="flex flex-col gap-4">
    <Table class="md:w-max">
      <TableBody>
        <TableRow>
          <TableHead>{{ t('attacker') }}</TableHead>
          <TableCell>
            <RouterLink v-if="toAttackerProfile" :to="toAttackerProfile">
              {{ report.attacker.id }}
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

        <TableRow class="hover:bg-card">
          <TableCell colspan="2">
            <Table>
              <TableHeader>
                <TableRow class="hover:bg-card">
                  <TableHead></TableHead>
                  <TableHead>{{ t('pikeman') }}</TableHead>
                  <TableHead>{{ t('swordsman') }}</TableHead>
                  <TableHead>{{ t('axeman') }}</TableHead>
                  <TableHead>{{ t('archer') }}</TableHead>
                  <TableHead>{{ t('light-cavalry') }}</TableHead>
                  <TableHead>{{ t('heavy-cavalry') }}</TableHead>
                </TableRow>
              </TableHeader>

              <TableBody>
                <TableRow class="hover:bg-card">
                  <TableCell>{{ t('quantity') }}</TableCell>
                  <TableCell>{{ report.result.attackerPersonnel.pikeman.size }}</TableCell>
                  <TableCell>{{ report.result.attackerPersonnel.swordsman.size }}</TableCell>
                  <TableCell>{{ report.result.attackerPersonnel.axeman.size }}</TableCell>
                  <TableCell>{{ report.result.attackerPersonnel.archer.size }}</TableCell>
                  <TableCell>{{ report.result.attackerPersonnel.lightCavalry.size }}</TableCell>
                  <TableCell>{{ report.result.attackerPersonnel.heavyCavalry.size }}</TableCell>
                </TableRow>

                <TableRow class="hover:bg-card">
                  <TableCell>{{ t('losses') }}</TableCell>
                  <TableCell>{{ losses.attackerLosses.pikeman.size }}</TableCell>
                  <TableCell>{{ losses.attackerLosses.swordsman.size }}</TableCell>
                  <TableCell>{{ losses.attackerLosses.axeman.size }}</TableCell>
                  <TableCell>{{ losses.attackerLosses.archer.size }}</TableCell>
                  <TableCell>{{ losses.attackerLosses.lightCavalry.size }}</TableCell>
                  <TableCell>{{ losses.attackerLosses.heavyCavalry.size }}</TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>

    <Table class="md:w-max">
      <TableBody>
        <TableRow>
          <TableHead>{{ t('defender') }}</TableHead>
          <TableCell>
            <RouterLink v-if="toDefenderProfile" :to="toDefenderProfile">
              {{ report.defender.id }}
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
            <Table>
              <TableHeader>
                <TableRow class="hover:bg-card">
                  <TableHead></TableHead>
                  <TableHead>{{ t('pikeman') }}</TableHead>
                  <TableHead>{{ t('swordsman') }}</TableHead>
                  <TableHead>{{ t('axeman') }}</TableHead>
                  <TableHead>{{ t('archer') }}</TableHead>
                  <TableHead>{{ t('light-cavalry') }}</TableHead>
                  <TableHead>{{ t('heavy-cavalry') }}</TableHead>
                </TableRow>
              </TableHeader>

              <TableBody>
                <TableRow class="hover:bg-card">
                  <TableCell>{{ t('quantity') }}</TableCell>
                  <TableCell>{{ report.result.defenderPersonnel.pikeman.size }}</TableCell>
                  <TableCell>{{ report.result.defenderPersonnel.swordsman.size }}</TableCell>
                  <TableCell>{{ report.result.defenderPersonnel.axeman.size }}</TableCell>
                  <TableCell>{{ report.result.defenderPersonnel.archer.size }}</TableCell>
                  <TableCell>{{ report.result.defenderPersonnel.lightCavalry.size }}</TableCell>
                  <TableCell>{{ report.result.defenderPersonnel.heavyCavalry.size }}</TableCell>
                </TableRow>

                <TableRow class="hover:bg-card">
                  <TableCell>{{ t('losses') }}</TableCell>
                  <TableCell>{{ losses.defenderLosses.pikeman.size }}</TableCell>
                  <TableCell>{{ losses.defenderLosses.swordsman.size }}</TableCell>
                  <TableCell>{{ losses.defenderLosses.axeman.size }}</TableCell>
                  <TableCell>{{ losses.defenderLosses.archer.size }}</TableCell>
                  <TableCell>{{ losses.defenderLosses.lightCavalry.size }}</TableCell>
                  <TableCell>{{ losses.defenderLosses.heavyCavalry.size }}</TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>

    <div v-if="!report.hauledResources.isEmpty()" class="flex items-center gap-4 pt-4">
      <div>{{ t('haul') }}</div>
      <div class="w-max max-w-max flex items-center gap-2">
        <Wood :amount="report.hauledResources.wood" />
        <Stone :amount="report.hauledResources.stone" />
        <Iron :amount="report.hauledResources.iron" />
        <Food :amount="report.hauledResources.food" />
      </div>
    </div>
  </div>
</template>
