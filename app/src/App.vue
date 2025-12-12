<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { throttle } from 'es-toolkit';
import { onKeyDown } from '@tb-dev/vue';
import { nextTick, onMounted } from 'vue';
import { handleError } from '@/lib/error';
import { Sonner } from '@tb-dev/vue-components';
import { setTheme, useSettings } from '@/settings';
import { createTrayIcon, showWindow } from '@/commands';
import { syncRef, useColorMode, watchImmediate } from '@vueuse/core';
import { defineGlobalCheats, defineGlobalCommands } from '@/lib/global';

const i18n = useI18n();

const settings = useSettings();
const { locale, theme, colorMode } = storeToRefs(settings);

watchImmediate(locale, setLocale);
watchImmediate(theme, setTheme);

syncRef(useColorMode(), colorMode, { direction: 'rtl' });

if (__DESKTOP__) {
  onKeyDown('F5', throttle(NIL.update, 1000));
}

onMounted(async () => {
  try {
    defineGlobalCommands();
    defineGlobalCheats();

    await nextTick();
    await createTrayIcon();
    await showWindow();
  }
  catch (err) {
    handleError(err);
  }
});

function setLocale(value: Locale) {
  i18n.locale.value = value;
}
</script>

<template>
  <main class="fixed inset-0 select-none pb-safe">
    <Sonner />
    <div class="relative size-full overflow-hidden">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
