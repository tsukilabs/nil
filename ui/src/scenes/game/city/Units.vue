<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';
import { useArmyPersonnelSize } from '@/composables/military/useArmyPersonnelSize';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableRow,
} from '@tb-dev/vue-components';

const props = defineProps<{ personnel: ArmyPersonnelImpl; }>();

const { t } = useI18n();

const size = useArmyPersonnelSize(() => props.personnel);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
        <span>{{ t('unit', 2) }}</span>
      </CardTitle>
    </CardHeader>

    <CardContent class="px-2 py-0">
      <Table>
        <TableBody>
          <TableRow v-if="size.pikeman > 0">
            <TableCell>
              {{ `${size.pikeman} ${t('pikeman', size.pikeman)}` }}
            </TableCell>
          </TableRow>

          <TableRow v-if="size.swordsman > 0">
            <TableCell>
              {{ `${size.swordsman} ${t('swordsman', size.swordsman)}` }}
            </TableCell>
          </TableRow>

          <TableRow v-if="size.axeman > 0">
            <TableCell>
              {{ `${size.axeman} ${t('axeman', size.axeman)}` }}
            </TableCell>
          </TableRow>

          <TableRow v-if="size.archer > 0">
            <TableCell>
              {{ `${size.archer} ${t('archer', size.archer)}` }}
            </TableCell>
          </TableRow>

          <TableRow v-if="size.lightCavalry > 0">
            <TableCell>
              {{ `${size.lightCavalry} ${t('light-cavalry', size.lightCavalry)}` }}
            </TableCell>
          </TableRow>

          <TableRow v-if="size.heavyCavalry > 0">
            <TableCell>
              {{ `${size.heavyCavalry} ${t('heavy-cavalry', size.heavyCavalry)}` }}
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
