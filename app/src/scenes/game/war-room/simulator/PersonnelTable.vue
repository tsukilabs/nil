<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import PersonnelTableRow from './PersonnelTableRow.vue';
import {
  NumberField,
  NumberFieldContent,
  NumberFieldInput,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const attacker = defineModel<ArmyPersonnel>('attacker', { required: true });
const defender = defineModel<ArmyPersonnel>('defender', { required: true });
const luck = defineModel<Luck>('luck', { required: true });
const wallLevel = defineModel<Option<BuildingLevel>>('wall', { required: true });

const { t } = useI18n();

const { stats } = NIL.world.refs();
</script>

<template>
  <Table class="md:w-max">
    <TableHeader>
      <TableRow class="hover:bg-card">
        <TableHead />
        <TableHead>{{ t('attacker') }}</TableHead>
        <TableHead>{{ t('defender') }}</TableHead>
      </TableRow>
    </TableHeader>

    <TableBody>
      <PersonnelTableRow
        v-model:attacker="attacker.pikeman"
        v-model:defender="defender.pikeman"
      />
      <PersonnelTableRow
        v-model:attacker="attacker.swordsman"
        v-model:defender="defender.swordsman"
      />
      <PersonnelTableRow
        v-model:attacker="attacker.axeman"
        v-model:defender="defender.axeman"
      />
      <PersonnelTableRow
        v-model:attacker="attacker.archer"
        v-model:defender="defender.archer"
      />
      <PersonnelTableRow
        v-model:attacker="attacker.lightCavalry"
        v-model:defender="defender.lightCavalry"
      />
      <PersonnelTableRow
        v-model:attacker="attacker.heavyCavalry"
        v-model:defender="defender.heavyCavalry"
      />
      <PersonnelTableRow
        v-model:attacker="attacker.ram"
        v-model:defender="defender.ram"
      />

      <div class="mt-4"></div>

      <TableRow>
        <TableCell>{{ t('wall') }}</TableCell>
        <TableCell />
        <TableCell>
          <NumberField
            v-model="wallLevel"
            :min="stats?.getBuildingMinLevel('wall')"
            :max="stats?.getBuildingMaxLevel('wall')"
            :step="1"
            :default-value="0"
          >
            <NumberFieldContent>
              <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:max-w-28 max-sm:text-xs" />
            </NumberFieldContent>
          </NumberField>
        </TableCell>
      </TableRow>

      <TableRow>
        <TableCell>{{ t('luck') }}</TableCell>
        <TableCell />
        <TableCell>
          <NumberField
            v-model="luck"
            :min="-20"
            :max="20"
            :step="1"
            :default-value="0"
          >
            <NumberFieldContent>
              <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:max-w-28 max-sm:text-xs" />
            </NumberFieldContent>
          </NumberField>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
