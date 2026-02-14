<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { throttle } from 'es-toolkit';
import { onKeyDown } from '@tb-dev/vue';
import Loading from '@/components/Loading.vue';
import CityTable from '@/components/CityTable.vue';
import { useContinentCities } from '@/composables/continent/useContinentCities';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n();

const { cities, loading, load, getDistance } = useContinentCities();

if (__DESKTOP__) {
  onKeyDown('F5', throttle(load, 1000));
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t('continent-cities') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="relative size-full overflow-auto px-2 py-0">
        <Loading v-if="loading" />
        <CityTable
          v-else-if="cities.length > 0"
          :cities
          show-owner
          :get-distance
          sortable
          default-sort-mode="distance"
          default-sort-order="asc"
          class="min-w-max"
        />
      </CardContent>
    </Card>
  </div>
</template>
