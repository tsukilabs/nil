<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { useI18n } from "@tsukilabs/nil-i18n";
import BattleResultTableRow from "./BattleResultTableRow.vue";
import type { BattleResultImpl } from "@/core/model/battle-result";
import { useBattleWallLevel } from "@/composables/battle/useBattleWallLevel";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@ui/table";

const props = defineProps<{ result: BattleResultImpl; }>();

const { t } = useI18n();

const wallLevel = useBattleWallLevel(() => props.result);
</script>

<template>
  <div class="w-full max-w-[90vw] flex flex-col gap-4">
    <Table class="min-w-max md:w-max pr-2">
      <TableHeader>
        <TableRow class="hover:bg-card">
          <TableHead />
          <TableHead>{{ t("pikeman") }}</TableHead>
          <TableHead>{{ t("swordsman") }}</TableHead>
          <TableHead>{{ t("axeman") }}</TableHead>
          <TableHead>{{ t("archer") }}</TableHead>
          <TableHead>{{ t("light-cavalry") }}</TableHead>
          <TableHead>{{ t("heavy-cavalry") }}</TableHead>
          <TableHead>{{ t("ram") }}</TableHead>
        </TableRow>
      </TableHeader>

      <TableBody>
        <BattleResultTableRow
          :head="t('war-room.attacker-units')"
          :personnel="result.attackerPersonnel"
        />

        <BattleResultTableRow
          :head="t('war-room.attacker-losses')"
          :personnel="result.getAttackerLosses()"
        />

        <BattleResultTableRow
          :head="t('war-room.defender-units')"
          :personnel="result.defenderPersonnel"
        />

        <BattleResultTableRow
          :head="t('war-room.defender-losses')"
          :personnel="result.getDefenderLosses()"
        />
      </TableBody>
    </Table>

    <Table v-if="wallLevel.didChange" class="md:w-max">
      <TableBody>
        <TableRow class="hover:bg-card">
          <TableHead>{{ t("report.wall-level") }}</TableHead>
          <TableCell>{{ wallLevel.original }} → {{ wallLevel.current }}</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
