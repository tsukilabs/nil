// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { createOnigurumaEngine } from 'shiki/engine/oniguruma';
import { createHighlighterCore, type HighlighterCore } from 'shiki/core';

interface Cache {
  highlighter: Option<HighlighterCore>;
}

const cache: Cache = {
  highlighter: null,
};

export async function createHighlighter() {
  cache.highlighter ??= await createHighlighterCore({
    engine: createOnigurumaEngine(import('shiki/wasm')),
    langs: [import('shiki/langs/lua.mjs')],
    themes: [
      import('shiki/themes/ayu-dark.mjs'),
      import('shiki/themes/ayu-light.mjs'),
    ],
  });

  return cache.highlighter;
}

export function toHtml(code: string) {
  return cache.highlighter?.codeToHtml(code, {
    lang: 'lua',
    theme: isDark() ? 'ayu-dark' : 'ayu-light',
  });
}

export function disposeHighlighter() {
  cache.highlighter?.dispose();
  cache.highlighter = null;
}

function isDark() {
  const html = document.querySelector('html');
  return html ? html.classList.contains('dark') : true;
}
