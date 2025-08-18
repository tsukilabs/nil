<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouteParams } from '@vueuse/router';
import type { RouteLocationAsRelative } from 'vue-router';
import enUS from '@/locale/en-US/scenes/game/profile/city.json';
import ptBR from '@/locale/pt-BR/scenes/game/profile/city.json';
import { usePublicCity } from '@/composables/city/usePublicCity';
import { usePublicCityOwner } from '@/composables/city/usePublicCityOwner';
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const continentKey = useRouteParams('ckey', null, { transform: Number.parseInt });
const { city, loading } = usePublicCity(continentKey);

const owner = computed(() => city.value?.owner);
const { bot, player, precursor } = usePublicCityOwner(owner);

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
      <CardHeader v-if="city && !loading">
        <CardTitle>
          <span>{{ city.name }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0 relative size-full">
        <div v-if="city">
          <Table class="sm:max-w-max">
            <TableBody>
              <TableRow>
                <TableHead>{{ t('coordinate', 2) }}</TableHead>
                <TableCell>{{ city.coord.format() }}</TableCell>
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
                <TableCell>{{ t(city.owner.kind) }}</TableCell>
              </TableRow>
            </TableBody>

            <TableFooter v-if="city">
              <TableRow class="bg-card hover:bg-card">
                <TableCell colspan="2" class="text-center">
                  <Button size="sm" :disabled="loading" @click="() => city?.goToContinent()">
                    <span>{{ t('show-on-map') }}</span>
                  </Button>
                </TableCell>
              </TableRow>
            </TableFooter>
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
