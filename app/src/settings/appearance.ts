// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BasicColorSchema } from '@vueuse/core';

export class AppearanceSettingsImpl implements AppearanceSettings {
  public colorMode: BasicColorSchema = 'dark';
  public theme: Theme = 'stone';

  public setTheme() {
    const html = document.querySelector('html');
    if (html) {
      const themeClass = `theme-${this.theme}`;
      html.classList.add(themeClass);

      for (const cl of Array.from(html.classList)) {
        if (cl !== themeClass && cl.startsWith('theme-')) {
          html.classList.remove(cl);
        }
      }
    }
  }
}
