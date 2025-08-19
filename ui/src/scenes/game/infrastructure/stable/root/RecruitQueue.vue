<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { formatInt } from '@/lib/intl';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import type { StableImpl } from '@/core/model/infrastructure/building/stable/stable';
import { Button, cn, Table, TableBody, TableCell, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  stable: StableImpl;
  loading: boolean;
  onCancel: (id: InfrastructureQueueOrderId) => MaybePromise<void>;
}>();

const { t } = useI18n();

const tableClass = computed(() => {
  return props.stable.recruitQueue.size === 0 ? 'hidden' : null;
});

const { sm } = useBreakpoints();

function formatSquad(squad: Squad) {
  return `${formatInt(squad.size)} ${t(squad.unit, squad.size)}`;
}
</script>

<template>
  <Table :class="cn(tableClass, 'xl:table xl:w-2/5 xl:max-w-[500px] xl:min-w-[250px]')">
    <TableBody>
      <template v-for="order of stable.recruitQueue" :key="order.id">
        <TableRow v-if="order.state.kind === 'pending'" class="hover:bg-card">
          <TableCell>
            <div class="flex items-center justify-start gap-2">
              <span>{{ formatSquad(order.squad) }}</span>
            </div>
          </TableCell>

          <TableCell>
            <div class="flex items-center justify-start">
              <Workforce :amount="order.state.workforce" />
            </div>
          </TableCell>

          <TableCell>
            <div class="flex items-center justify-start md:justify-center">
              <Button
                variant="destructive"
                :size="sm ? 'sm' : 'xs'"
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
