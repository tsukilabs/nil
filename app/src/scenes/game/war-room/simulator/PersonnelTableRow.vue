<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { TableCell, TableRow } from '@ui/table';
import type { Squad } from '@/types/core/military/squad';
import { NumberField, NumberFieldContent, NumberFieldInput } from '@ui/number-field';

const attacker = defineModel<Squad>('attacker', { required: true });
const defender = defineModel<Squad>('defender', { required: true });

const { t } = useI18n();
</script>

<template>
  <TableRow>
    <TableCell>{{ t(attacker.unit) }}</TableCell>
    <TableCell>
      <NumberField
        v-model="attacker.size"
        :min="0"
        :step="1"
        :default-value="0"
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:max-w-28 max-sm:text-xs" />
        </NumberFieldContent>
      </NumberField>
    </TableCell>
    <TableCell>
      <NumberField
        v-model="defender.size"
        :min="0"
        :step="1"
        :default-value="0"
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldInput class="dark:bg-input/40 max-sm:h-6 max-sm:max-w-28 max-sm:text-xs" />
        </NumberFieldContent>
      </NumberField>
    </TableCell>
  </TableRow>
</template>
