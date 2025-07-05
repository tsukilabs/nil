<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { onMounted } from 'vue';
import { throttle } from 'es-toolkit';
import { showWindow } from '@/commands';
import { onKeyDown } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import { useColorMode } from '@vueuse/core';
import { Sonner } from '@tb-dev/vue-components';
import { defineGlobalCommands } from '@/lib/console';

useColorMode({
  storageKey: 'nil:color-mode',
  initialValue: 'dark',
  writeDefaults: true,
  onError: handleError,
});

onKeyDown('F5', throttle(NIL.update, 1000));

onMounted(async () => {
  try {
    await showWindow();
    await defineGlobalCommands();
  } catch (err) {
    handleError(err);
  }
});
</script>

<template>
  <main class="fixed inset-0 select-none">
    <Sonner />
    <div class="absolute inset-0 overflow-hidden">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
