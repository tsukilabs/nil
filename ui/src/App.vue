<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { throttle } from 'es-toolkit';
import type { Locale } from '@/locale';
import { showWindow } from '@/commands';
import { onKeyDown } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import { setTheme } from '@/lib/settings';
import { watchImmediate } from '@vueuse/core';
import { Sonner } from '@tb-dev/vue-components';
import { defineGlobalCommands } from '@/lib/global';
import { useSettingsStore } from '@/stores/settings';

const i18n = useI18n();

const settings = useSettingsStore();
const { locale, theme } = storeToRefs(settings);

watchImmediate(locale, setLocale);
watchImmediate(theme, setTheme);

onKeyDown('F5', throttle(NIL.update, 1000));

onMounted(async () => {
  try {
    await showWindow();
    defineGlobalCommands();
  } catch (err) {
    handleError(err);
  }
});

function setLocale(value: Locale) {
  i18n.locale.value = value;
}
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
