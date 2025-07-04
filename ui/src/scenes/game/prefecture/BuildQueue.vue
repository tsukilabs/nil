<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { ChevronDownIcon, ChevronUpIcon } from 'lucide-vue-next';
import type { PrefectureImpl } from '@/core/model/buildings/prefecture';
import { Button, Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  prefecture: PrefectureImpl;
  loading: boolean;
}>();

const last = computed(() => props.prefecture.buildQueue.last());
</script>

<template>
  <Table>
    <template #header>
      <TableRow class="bg-background hover:bg-background">
        <TableHead>
          <span>{{ $t('order') }}</span>
        </TableHead>
        <TableHead>
          <span>{{ $t('workforce') }}</span>
        </TableHead>
        <TableHead>
          <span></span>
        </TableHead>
      </TableRow>
    </template>

    <TableRow v-for="order of prefecture.buildQueue" :key="order.id">
      <TableCell>
        <div class="flex items-center justify-start gap-2">
          <ChevronUpIcon
            v-if="order.kind === 'construction'"
            color="#00bd7e"
            stroke-width="2px"
            class="size-5"
          />
          <ChevronDownIcon
            v-else-if="order.kind === 'demolition'"
            color="#e61001"
            stroke-width="2px"
            class="size-5"
          />

          <span>{{ `${$t(order.building)} (${$t('level-x', [order.level])})` }}</span>
        </div>
      </TableCell>
      <TableCell>
        <div class="flex items-center justify-start">
          <Workforce :amount="order.workforce" />
        </div>
      </TableCell>
      <TableCell>
        <div v-if="order.id === last?.id" class="flex items-center justify-center">
          <Button variant="destructive" size="sm" :disabled="loading">
            <span>{{ $t('cancel') }}</span>
          </Button>
        </div>
      </TableCell>
    </TableRow>
  </Table>
</template>
