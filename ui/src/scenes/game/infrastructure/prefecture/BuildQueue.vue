<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { MaybePromise } from '@tb-dev/utils';
import { ChevronDownIcon, ChevronUpIcon } from 'lucide-vue-next';
import type { PrefectureImpl } from '@/core/model/buildings/prefecture';
import { Button, cn, Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  prefecture: PrefectureImpl;
  loading: boolean;
  onCancel: () => MaybePromise<void>;
}>();

const { t } = useI18n();

const last = computed(() => props.prefecture.buildQueue.last());

const tableClass = computed(() => {
  return props.prefecture.buildQueue.size === 0 ? 'hidden' : null;
});
</script>

<template>
  <Table :class="cn(tableClass, 'xl:table xl:w-2/5 xl:max-w-[500px] xl:min-w-[250px]')">
    <template #header>
      <TableRow class="bg-background hover:bg-background">
        <TableHead>
          <span>{{ t('order') }}</span>
        </TableHead>
        <TableHead>
          <span>{{ t('workforce') }}</span>
        </TableHead>
        <TableHead>
          <span></span>
        </TableHead>
      </TableRow>
    </template>

    <template v-for="order of prefecture.buildQueue" :key="order.id">
      <TableRow v-if="order.status.kind === 'pending'">
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

            <span>{{ `${t(order.building)} (${t('level-x', [order.level])})` }}</span>
          </div>
        </TableCell>
        <TableCell>
          <div class="flex items-center justify-start">
            <Workforce :amount="order.status.workforce" />
          </div>
        </TableCell>
        <TableCell>
          <div v-if="order.id === last?.id" class="flex items-center justify-center">
            <Button variant="destructive" size="sm" :disabled="loading" @click="onCancel">
              <span>{{ t('cancel') }}</span>
            </Button>
          </div>
        </TableCell>
      </TableRow>
    </template>
  </Table>
</template>
