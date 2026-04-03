<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { toMerged } from 'es-toolkit/object';
import BattleResultTableRow from './BattleResultTableRow.vue';
import enUS_report from '@/locale/en-US/scenes/game/report.json';
import ptBR_report from '@/locale/pt-BR/scenes/game/report.json';
import type { BattleResultImpl } from '@/core/model/battle-result';
import enUS_warRoom from '@/locale/en-US/scenes/game/war-room.json';
import ptBR_warRoom from '@/locale/pt-BR/scenes/game/war-room.json';
import { useBattleWallLevel } from '@/composables/battle/useBattleWallLevel';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{ result: BattleResultImpl; }>();

const { t } = useI18n({
  messages: {
    'en-US': toMerged(enUS_report, enUS_warRoom),
    'pt-BR': toMerged(ptBR_report, ptBR_warRoom),
  },
});

const wallLevel = useBattleWallLevel(() => props.result);
</script>

<template>
  <div class="w-full min-w-max flex flex-col gap-4">
    <Table class="md:w-max">
      <TableHeader>
        <TableRow class="hover:bg-card">
          <TableHead />
          <TableHead>{{ t('pikeman') }}</TableHead>
          <TableHead>{{ t('swordsman') }}</TableHead>
          <TableHead>{{ t('axeman') }}</TableHead>
          <TableHead>{{ t('archer') }}</TableHead>
          <TableHead>{{ t('light-cavalry') }}</TableHead>
          <TableHead>{{ t('heavy-cavalry') }}</TableHead>
          <TableHead>{{ t('ram') }}</TableHead>
        </TableRow>
      </TableHeader>

      <TableBody>
        <BattleResultTableRow
          :head="t('attacker-units')"
          :personnel="result.attackerPersonnel"
        />

        <BattleResultTableRow
          :head="t('attacker-losses')"
          :personnel="result.getAttackerLosses()"
        />

        <BattleResultTableRow
          :head="t('defender-units')"
          :personnel="result.defenderPersonnel"
        />

        <BattleResultTableRow
          :head="t('defender-losses')"
          :personnel="result.getDefenderLosses()"
        />
      </TableBody>
    </Table>

    <Table v-if="wallLevel.didChange" class="md:w-max">
      <TableBody>
        <TableRow class="hover:bg-card">
          <TableHead>{{ t('wall-level') }}</TableHead>
          <TableCell>{{ wallLevel.original }} → {{ wallLevel.current }}</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
