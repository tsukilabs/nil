<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { throttle } from 'es-toolkit';
import type { Locale } from '@/locale';
import { onKeyDown } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import { Sonner } from '@tb-dev/vue-components';
import { setTheme, useSettings } from '@/settings';
import { createTrayIcon, showWindow } from '@/commands';
import { syncRef, useColorMode, watchImmediate } from '@vueuse/core';
import { defineGlobalCommands, defineReactiveConsole } from '@/lib/global';

const i18n = useI18n();

const settings = useSettings();
const { locale, theme, colorMode } = storeToRefs(settings);

const desktop = globalThis.__DESKTOP__;

watchImmediate(locale, setLocale);
watchImmediate(theme, setTheme);

syncRef(useColorMode(), colorMode, { direction: 'rtl' });

onKeyDown('F5', throttle(NIL.update, 1000), { enabled: desktop });

onMounted(async () => {
  try {
    await createTrayIcon();
    await showWindow();
    defineGlobalCommands();
    defineReactiveConsole();
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
