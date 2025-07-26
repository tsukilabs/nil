<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { ResourcesImpl } from '@/core/model/resources';
import { useStableSettings } from '@/settings/infrastructure/stable';
import type { StableImpl } from '@/core/model/infrastructure/building/stable';
import { useRecruitCatalogEntry } from '@/composables/infrastructure/useRecruitCatalogEntry';
import {
  Button,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
  TableCell,
  TableRow,
} from '@tb-dev/vue-components';

const props = defineProps<{
  entry: StableRecruitCatalogEntry;
  unit: StableUnitId;
  stable: StableImpl;
  loading: boolean;
  isPlayerTurn: boolean;
  playerResources: Option<ResourcesImpl>;
  onRecruitOrder: (chunks: number) => void;
}>();

const { t } = useI18n();

const { player } = NIL.player.refs();

const settings = useStableSettings();
const { hideUnmet } = storeToRefs(settings);

const { chunks, resources, maintenance, workforce } = useRecruitCatalogEntry(() => props.entry);

const canRecruit = computed(() => {
  if (
    !props.loading &&
    player.value &&
    props.isPlayerTurn &&
    props.stable.enabled &&
    props.entry.kind === 'available'
  ) {
    return player.value.hasResources(resources.value);
  }

  return false;
});
</script>

<template>
  <TableRow v-if="entry.kind === 'available'">
    <TableCell class="min-w-24">
      <span>{{ t(unit) }}</span>
    </TableCell>
    <TableCell>
      <div class="grid grid-cols-3 items-center justify-start gap-4">
        <Wood :amount="resources.wood" :limit="playerResources?.wood" />
        <Stone :amount="resources.stone" :limit="playerResources?.stone" />
        <Iron :amount="resources.iron" :limit="playerResources?.iron" />
      </div>
    </TableCell>
    <TableCell>
      <div class="flex items-center justify-start">
        <Food :amount="maintenance" />
      </div>
    </TableCell>
    <TableCell>
      <div class="flex items-center justify-start">
        <Workforce :amount="workforce" />
      </div>
    </TableCell>
    <TableCell class="min-w-30">
      <div class="grid max-w-fit grid-cols-2 items-center justify-start gap-4">
        <NumberField
          v-model="chunks"
          :disabled="loading"
          :min="0"
          :max="9999"
          :step="1"
          class="w-full"
        >
          <NumberFieldContent>
            <NumberFieldDecrement />
            <NumberFieldInput class="dark:bg-input/40" />
            <NumberFieldIncrement />
          </NumberFieldContent>
        </NumberField>

        <Button
          variant="default"
          size="sm"
          :disabled="!canRecruit || chunks < 1"
          class="max-w-24"
          @click="() => onRecruitOrder(Math.trunc(chunks))"
        >
          <span>{{ t('recruit') }}</span>
        </Button>
      </div>
    </TableCell>
  </TableRow>

  <TableRow v-else-if="entry.kind === 'unmet' && !hideUnmet">
    <TableCell class="min-w-24">
      <span>{{ t(unit) }}</span>
    </TableCell>
    <TableCell colspan="4" class="w-full">
      <div class="text-muted-foreground flex w-full items-center justify-center text-sm">
        <span>{{ t('not-yet-available') }}</span>
      </div>
    </TableCell>
  </TableRow>

  <TableRow v-else class="hidden" />
</template>
