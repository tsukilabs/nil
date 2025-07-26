<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { StableImpl } from '@/core/model/infrastructure/building/stable';
import { Button, cn, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  stable: StableImpl;
  loading: boolean;
  onCancel: (id: InfrastructureQueueOrderId) => MaybePromise<void>;
}>();

const { t } = useI18n();

const tableClass = computed(() => {
  return props.stable.recruitQueue.size === 0 ? 'hidden' : null;
});

function format(squad: Squad) {
  return `${squad.size} ${t(squad.unit, squad.size)}`;
}
</script>

<template>
  <Table :class="cn(tableClass, 'xl:table xl:w-2/5 xl:max-w-[500px] xl:min-w-[250px]')">
    <TableHeader>
      <TableRow class="bg-card">
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
    </TableHeader>

    <TableBody>
      <template v-for="order of stable.recruitQueue" :key="order.id">
        <TableRow v-if="order.state.kind === 'pending'">
          <TableCell>
            <div class="flex items-center justify-start gap-2">
              <span>{{ format(order.squad) }}</span>
            </div>
          </TableCell>
          <TableCell>
            <div class="flex items-center justify-start">
              <Workforce :amount="order.state.workforce" />
            </div>
          </TableCell>
          <TableCell>
            <div class="flex items-center justify-center">
              <Button
                variant="destructive"
                size="sm"
                :disabled="loading"
                @click="() => onCancel(order.id)"
              >
                <span>{{ t('cancel') }}</span>
              </Button>
            </div>
          </TableCell>
        </TableRow>
      </template>
    </TableBody>
  </Table>
</template>
