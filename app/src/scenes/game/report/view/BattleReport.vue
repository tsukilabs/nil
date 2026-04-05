<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import Food from '@/components/resources/Food.vue';
import Iron from '@/components/resources/Iron.vue';
import Wood from '@/components/resources/Wood.vue';
import Stone from '@/components/resources/Stone.vue';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { useBattleLosses } from '@/composables/battle/useBattleLosses';
import type { BattleReportImpl } from '@/core/model/report/battle-report';
import { useBattleWallLevel } from '@/composables/battle/useBattleWallLevel';
import { Table, TableBody, TableCell, TableHead, TableRow } from '@ui/table';
import BattleReportTable from '@/scenes/game/report/view/BattleReportTable.vue';

const props = defineProps<{
  report: BattleReportImpl;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const result = computed(() => props.report.result);
const losses = useBattleLosses(result);
const wallLevel = useBattleWallLevel(result);
</script>

<template>
  <div class="w-full min-w-max flex flex-col gap-4">
    <BattleReportTable
      kind="attacker"
      :winner="report.result.winner"
      :ruler="report.attacker"
      :personnel="report.result.attackerPersonnel"
      :losses="losses.attackerLosses"
      :city="report.originCity"
    />

    <BattleReportTable
      kind="defender"
      :winner="report.result.winner"
      :ruler="report.defender"
      :personnel="report.result.defenderPersonnel"
      :losses="losses.defenderLosses"
      :city="report.destinationCity"
    />

    <Table class="max-w-[800px]">
      <TableBody>
        <TableRow class="hover:bg-card">
          <TableHead>{{ t('attacker-luck') }}</TableHead>
          <TableCell>{{ report.result.formatLuck() }}</TableCell>
        </TableRow>

        <TableRow v-if="report.result.wallLevel > 0" class="hover:bg-card">
          <TableHead>{{ t('wall-level') }}</TableHead>
          <TableCell v-if="wallLevel.didChange">
            {{ wallLevel.original }} → {{ wallLevel.current }}
          </TableCell>
          <TableCell v-else>
            {{ wallLevel.current }}
          </TableCell>
        </TableRow>

        <TableRow v-if="!report.hauledResources.isEmpty()" class="hover:bg-card">
          <TableHead>{{ t('haul') }}</TableHead>
          <TableCell>
            <div class="w-max max-w-max flex items-center gap-4">
              <Wood :amount="report.hauledResources.wood" />
              <Stone :amount="report.hauledResources.stone" />
              <Iron :amount="report.hauledResources.iron" />
              <Food :amount="report.hauledResources.food" />
            </div>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
