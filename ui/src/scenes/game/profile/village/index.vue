<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Loading from '@/components/Loading.vue';
import { useRouteParams } from '@vueuse/router';
import { usePublicVillage } from '@/composables/village/usePublicVillage';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const ckey = useRouteParams('ckey', null, { transform: Number.parseInt });
const { village, loading } = usePublicVillage(ckey);
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader v-if="village && !loading">
        <CardTitle>
          <span>{{ village.name }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0 relative size-full">
        <Loading v-if="!village || loading" class="absolute inset-0" />
        <pre v-else>{{ JSON.stringify(village, null, 4) }}</pre>
      </CardContent>
    </Card>
  </div>
</template>
