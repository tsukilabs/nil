// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

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
