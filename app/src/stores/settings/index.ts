// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';
import type { BasicColorSchema } from '@vueuse/core';
import { type as osType } from '@tauri-apps/plugin-os';

export const useSettings = defineStore('settings', () => {
  const locale = ref<Locale>(getDefaultLocale());
  const theme = ref<Theme>('stone');
  const colorMode = ref<BasicColorSchema>('dark');

  const hideOnClose = ref(false);
  const autoUpdate = ref(osType() === 'windows');

  return {
    locale,
    theme,
    colorMode,
    hideOnClose,
    autoUpdate,
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

function getDefaultLocale(): Locale {
  if (window.navigator.language.startsWith('pt')) {
    return 'pt-BR';
  }
  else {
    return 'en-US';
  }
}
