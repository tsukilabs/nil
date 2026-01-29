<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useUserStore } from '@/stores/user';
import Loading from '@/components/Loading.vue';

const userStore = useUserStore();
await userStore.validateToken();
</script>

<template>
  <RouterView #default="{ Component }">
    <template v-if="Component">
      <Suspense>
        <component :is="Component" />
        <template #fallback>
          <Loading class="absolute inset-0" />
        </template>
      </Suspense>
    </template>
  </RouterView>
</template>
