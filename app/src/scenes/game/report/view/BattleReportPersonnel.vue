<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import enUS from '@/locale/en-US/scenes/game/report.json';
import ptBR from '@/locale/pt-BR/scenes/game/report.json';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  personnel: ArmyPersonnel;
  losses: ArmyPersonnel;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow class="hover:bg-card">
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
      <TableRow class="hover:bg-card">
        <TableCell>{{ personnel.pikeman.size }}</TableCell>
        <TableCell>{{ personnel.swordsman.size }}</TableCell>
        <TableCell>{{ personnel.axeman.size }}</TableCell>
        <TableCell>{{ personnel.archer.size }}</TableCell>
        <TableCell>{{ personnel.lightCavalry.size }}</TableCell>
        <TableCell>{{ personnel.heavyCavalry.size }}</TableCell>
        <TableCell>{{ personnel.ram.size }}</TableCell>
      </TableRow>

      <TableRow class="hover:bg-card">
        <TableCell>{{ losses.pikeman.size }}</TableCell>
        <TableCell>{{ losses.swordsman.size }}</TableCell>
        <TableCell>{{ losses.axeman.size }}</TableCell>
        <TableCell>{{ losses.archer.size }}</TableCell>
        <TableCell>{{ losses.lightCavalry.size }}</TableCell>
        <TableCell>{{ losses.heavyCavalry.size }}</TableCell>
        <TableCell>{{ losses.ram.size }}</TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
