<script setup lang="ts">
import { go } from '@/router';
import { onMounted } from 'vue';
import { handleError } from '@/lib/error';
import { useColorMode } from '@vueuse/core';
import { isDev, showWindow } from '@/commands';
import { onCtrlKeyDown } from '@/composables/key-down';

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  storageKey: 'nil:color-mode',
  writeDefaults: true,
});

onCtrlKeyDown('Home', async () => {
  if (await isDev()) go('home');
});

onMounted(() => showWindow().handleError());
</script>

<template>
  <main class="fixed inset-0 select-none">
    <div class="absolute inset-0 overflow-hidden">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
