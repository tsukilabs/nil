// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { defineStore } from 'pinia';
import { localRef } from '@tb-dev/vue';
import type { Locale } from '@/locale';
import { handleError } from '@/lib/error';
import { useColorMode } from '@vueuse/core';

export const useSettingsStore = defineStore('settings', () => {
  const locale = localRef<Locale>('nil:locale', 'en-US');
  const theme = localRef<Theme>('nil:theme', 'zinc');

  const colorMode = useColorMode({
    storageKey: 'nil:color-mode',
    initialValue: 'dark',
    writeDefaults: true,
    onError: handleError,
  });

  return {
    locale,
    theme,
    colorMode,
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
