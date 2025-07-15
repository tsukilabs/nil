// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import * as monaco from 'monaco-editor-core';
import { shikiToMonaco } from '@shikijs/monaco';
import { createOnigurumaEngine } from 'shiki/engine/oniguruma';
import type { editor as MonacoEditor } from 'monaco-editor-core';
import { createHighlighterCore, type HighlighterCore } from 'shiki/core';
import EditorWorker from 'monaco-editor-core/esm/vs/editor/editor.worker?worker';

export type CodeEditor = MonacoEditor.IStandaloneCodeEditor;

(self as any).MonacoEnvironment = {
  getWorker: () => new EditorWorker(),
};

interface Cache {
  editor: Option<CodeEditor>;
  highlighter: Option<HighlighterCore>;
}

const cache: Cache = {
  editor: null,
  highlighter: null,
};

export async function createHighlighter() {
  cache.highlighter ??= await createHighlighterCore({
    engine: createOnigurumaEngine(import('shiki/wasm')),
    langs: [import('shiki/langs/lua.mjs')],
    themes: [import('shiki/themes/vesper.mjs')],
  });

  return cache.highlighter;
}

export async function createEditor(element: HTMLElement) {
  // Ensure highlighter is created.
  const highlighter = await createHighlighter();

  if (!cache.editor) {
    const languages = monaco.languages.getLanguages();
    if (languages.every((it) => it.id !== 'lua')) {
      monaco.languages.register({ id: 'lua' });
    }

    shikiToMonaco(highlighter, monaco);

    cache.editor = monaco.editor.create(element, {
      language: 'lua',
      theme: 'vesper',
      value: '',
      minimap: { enabled: false },
    });
  }

  return cache.editor;
}

export function dispose() {
  disposeEditor();
  disposeHighlighter();
}

export function disposeEditor() {
  cache.editor?.dispose();
  cache.editor = null;
}

export function disposeHighlighter() {
  cache.highlighter?.dispose();
  cache.highlighter = null;
}
