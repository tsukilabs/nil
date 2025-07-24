// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { compare } from '@/lib/intl';
import * as commands from '@/commands';
import * as dialog from '@/lib/dialog';
import { useMutex } from '@tb-dev/vue';
import type { CodeEditor } from '@/lib/editor';
import { ref, type Ref, shallowRef } from 'vue';

export function useScripts(editor: Ref<Option<CodeEditor>>) {
  const scripts = shallowRef<Script[]>([]);
  const current = ref<Option<Script>>();
  const { locked, lock } = useMutex();

  const { player } = NIL.player.refs();

  async function loadScripts() {
    const it = await commands.getScripts();
    it.sort((a, b) => compare(a.name, b.name));
    scripts.value = it;
  }

  async function executeScript() {
    await saveScript();
    await lock(async () => {
      if (current.value) {
        await commands.executeScript(current.value.id);
      }
    });
  }

  async function saveScript() {
    await lock(async () => {
      if (current.value) {
        if (editor.value) {
          current.value.code = editor.value.getValue();
        }

        const id = current.value.id;
        await commands.updateScript(current.value);

        const script = await commands.getScript(id);
        if (current.value.id === script.id) {
          setCurrent(script);
        }

        const index = scripts.value.findIndex((it) => {
          return it.id === script.id;
        });

        if (index !== -1) {
          scripts.value = scripts.value.toSpliced(index, 1, script);
        }
      }
    });
  }

  async function createScript() {
    await lock(async () => {
      if (player.value) {
        const id = await commands.addScript({
          name: 'Script',
          code: '\n',
          owner: player.value.id,
        });

        const script = await commands.getScript(id);
        setCurrent(script);
        scripts.value.push(script);
        scripts.value = scripts.value.toSorted((a, b) => compare(a.name, b.name));
      }
    });
  }

  async function removeScript() {
    await lock(async () => {
      if (current.value) {
        const id = current.value.id;
        await commands.removeScript(id);
        if (current.value.id === id) {
          current.value = null;
        }

        scripts.value = scripts.value.filter((script) => {
          return script.id !== id;
        });
      }
    });
  }

  async function importScripts() {
    await lock(async () => {
      const files = await dialog.open({
        directory: false,
        multiple: true,
        filters: [{ name: 'Lua', extensions: ['lua'] }],
      });

      if (Array.isArray(files) && files.length > 0) {
        const ids = await commands.importScripts(files);
        if (ids.length > 0) await loadScripts();
      }
    });
  }

  async function exportScripts() {
    await lock(async () => {
      if (current.value) {
        const script = current.value;
        const dir = await dialog.open({
          directory: true,
          multiple: false,
        });

        if (dir) {
          await commands.exportScript(dir, script.name, script.code);
        }
      }
    });
  }

  function setCurrent(script: Script) {
    if (editor.value && current.value) {
      current.value.code = editor.value.getValue();
    }

    current.value = script;
  }

  return {
    scripts,
    current,
    loading: locked,
    loadScripts,
    executeScript,
    saveScript,
    createScript,
    removeScript,
    importScripts,
    exportScripts,
    setCurrent,
  };
}
