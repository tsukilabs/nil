// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';
import type { Locale } from '@/locale';
import type { BasicColorSchema } from '@vueuse/core';

export const useSettings = defineStore('settings', () => {
  const locale = ref<Locale>('en-US');
  const theme = ref<Theme>('stone');
  const colorMode = ref<BasicColorSchema>('dark');

  const hideOnClose = ref(false);

  return {
    locale,
    theme,
    colorMode,
    hideOnClose,
  };
});

export function setTheme(theme: Theme) {
  const html = document.querySelector('html');
  if (html) {
    const themeClass = `theme-${theme}`;
    html.classList.add(themeClass);

    for (const cl of Array.from(html.classList)) {
      if (cl !== themeClass && cl.startsWith('theme-')) {
        html.classList.remove(cl);
      }
    }
  }
}
