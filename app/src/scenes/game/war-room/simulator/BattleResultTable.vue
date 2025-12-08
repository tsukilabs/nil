<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/game/war-room.json';
import ptBR from '@/locale/pt-BR/scenes/game/war-room.json';
import BattleResultTableRow from './BattleResultTableRow.vue';
import type { BattleResultImpl } from '@/core/model/battle-result';
import { Table, TableBody, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

defineProps<{ result: BattleResultImpl; }>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});
</script>

<template>
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
</template>
