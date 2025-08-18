<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouteParams } from '@vueuse/router';
import type { RouteLocationAsRelative } from 'vue-router';
import { usePublicVillage } from '@/composables/village/usePublicVillage';
import { usePublicVillageOwner } from '@/composables/village/usePublicVillageOwner';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const continentKey = useRouteParams('ckey', null, { transform: Number.parseInt });
const { village, loading } = usePublicVillage(continentKey);

const owner = computed(() => village.value?.owner);
const { bot, player, precursor } = usePublicVillageOwner(owner);

const toOwnerScene = computed<Option<RouteLocationAsRelative>>(() => {
  if (owner.value) {
    const kind = owner.value.kind;
    return {
      name: `profile-${kind}` satisfies ProfileScene,
      params: { id: String(owner.value.id) },
    };
  }

  return null;
});
</script>

<template>
  <div class="game-layout">
    <Card class="size-full overflow-x-hidden overflow-y-auto">
      <CardHeader v-if="village && !loading">
        <CardTitle>
          <span>{{ village.name }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0 relative size-full">
        <div v-if="village">
          <Table class="sm:max-w-max">
            <TableBody>
              <TableRow>
                <TableHead>{{ t('coordinate', 2) }}</TableHead>
                <TableCell>{{ village.coord.format() }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('point', 2) }}</TableHead>
                <TableCell>???</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('owner') }}</TableHead>
                <TableCell>
                  <RouterLink v-if="toOwnerScene" :to="toOwnerScene">
                    <span v-if="bot">{{ bot.name }}</span>
                    <span v-else-if="player">{{ player.id }}</span>
                    <span v-else-if="precursor">{{ precursor.id }}</span>
                  </RouterLink>
                </TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('type') }}</TableHead>
                <TableCell>{{ t(village.owner.kind) }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
:deep(table :where(th, td)) {
  padding-right: 2rem;
}
</style>
