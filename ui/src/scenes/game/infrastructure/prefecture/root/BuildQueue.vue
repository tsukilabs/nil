<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { ChevronDownIcon, ChevronUpIcon } from 'lucide-vue-next';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';
import type { PrefectureImpl } from '@/core/model/infrastructure/building/prefecture/prefecture';
import { Button, cn, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const props = defineProps<{
  prefecture: PrefectureImpl;
  loading: boolean;
  onCancel: () => MaybePromise<void>;
}>();

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const last = computed(() => props.prefecture.buildQueue.last());

const tableClass = computed(() => {
  return props.prefecture.buildQueue.size === 0 ? 'hidden' : null;
});
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
      <template v-for="order of prefecture.buildQueue" :key="order.id">
        <TableRow
          v-if="order.state.kind === 'pending' && order.state.workforce > 0"
          @dblclick="onCancel"
        >
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
              <div class="flex flex-col gap-1">
                <span class="md:whitespace-nowrap">
                  {{ t(order.building) }}
                </span>
                <span class="text-muted-foreground text-xs">
                  {{ t('level-x', [order.level]) }}
                </span>
              </div>
            </div>
          </TableCell>

          <TableCell>
            <div class="flex items-center justify-start">
              <Workforce :amount="order.state.workforce" />
            </div>
          </TableCell>

          <TableCell>
            <div v-if="order.id === last?.id" class="flex items-center justify-start md:justify-center">
              <Button
                variant="destructive"
                size="sm"
                :disabled="loading"
                @click="onCancel"
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
