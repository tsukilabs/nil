<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { formatInt } from '@/lib/intl';
import { usePublicCity } from '@/composables/city/usePublicCity';
import type { PublicFieldImpl } from '@/core/model/continent/public-field';
import { useCityOwnerSceneLink } from '@/composables/city/useCityOwnerSceneLink';
import { useCityProfileSceneLink } from '@/composables/city/useCityProfileSceneLink';
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@tb-dev/vue-components';

const props = defineProps<{
  field: PublicFieldImpl;
}>();

const { t } = useI18n();

const { city } = usePublicCity(() => props.field.coord);
const owner = computed(() => city.value?.owner);

const toOwnerScene = useCityOwnerSceneLink(owner);
const toCityProfile = useCityProfileSceneLink(city);

const color = computed(() => {
  switch (city.value?.owner.kind) {
    case 'bot': {
      return 'bg-amber-950';
    }
    case 'player': {
      return 'bg-primary';
    }
    case 'precursor': {
      return getPrecursorColor(city.value.owner.id);
    }
    default:
      return 'bg-transparent';
  }
});

function getPrecursorColor(id: PrecursorId) {
  switch (id) {
    case 'A': {
      return 'bg-green-900';
    }
    case 'B': {
      return 'bg-purple-900';
    }
  }
}
</script>

<template>
  <HoverCard :open-delay="200" :close-delay="100">
    <HoverCardTrigger as-child>
      <div class="absolute inset-0 flex items-center justify-center overflow-hidden select-none">
        <RouterLink
          v-if="toCityProfile"
          :to="toCityProfile"
          :class="color"
          class="size-[75%] cursor-pointer rounded-full"
        />
      </div>
    </HoverCardTrigger>

    <HoverCardContent>
      <div class="flex flex-col select-none">
        <div v-if="city && owner" class="flex flex-col gap-2 md:gap-4 overflow-hidden">
          <h1 class="ellipsis text-lg text-center">
            <RouterLink v-if="toCityProfile" :to="toCityProfile">{{ city.name }}</RouterLink>
          </h1>
          <Table class="w-full">
            <TableBody>
              <TableRow>
                <TableHead>{{ t('coordinate', 2) }}</TableHead>
                <TableCell>{{ city.coord.format() }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('point', 2) }}</TableHead>
                <TableCell>{{ formatInt(city.score) }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('owner') }}</TableHead>
                <TableCell>
                  <RouterLink v-if="toOwnerScene" :to="toOwnerScene">
                    <span class="whitespace-pre-wrap wrap-anywhere">{{ owner.id }}</span>
                  </RouterLink>
                </TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('type') }}</TableHead>
                <TableCell>{{ t(city.owner.kind) }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
      </div>
    </HoverCardContent>
  </HoverCard>
</template>
