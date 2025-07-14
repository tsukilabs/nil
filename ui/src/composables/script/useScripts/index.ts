// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { compare } from '@/lib/intl';
import * as commands from '@/commands';
import * as dialog from '@/lib/dialog';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import type { CodeEditor } from '@/lib/editor';
import { readonly, ref, type Ref, shallowRef } from 'vue';

export function useScripts(editor: Ref<Option<CodeEditor>>) {
  const scripts = shallowRef<Script[]>([]);
  const current = ref<Option<Script>>();
  const loading = ref(false);

  const { player } = NIL.player.refs();

  async function loadScripts() {
    const it = await commands.getScripts();
    it.sort((a, b) => compare(a.name, b.name));
    scripts.value = it;
  }

  async function executeScript() {
    if (current.value && !loading.value) {
      try {
        await saveScript();
        loading.value = true;
        await commands.executeScript(current.value.id);
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function saveScript() {
    if (current.value && !loading.value) {
      try {
        loading.value = true;
        if (editor.value) {
          current.value.code = editor.value.getValue();
        }

        if (current.value.id > 0) {
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
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function createScript() {
    if (player.value && !loading.value) {
      try {
        loading.value = true;
        const empty = createEmptyScript(player.value.id);
        const id = await commands.addScript(empty);

        const script = await commands.getScript(id);
        script.name += ` ${id}`;
        setCurrent(script);
        scripts.value.push(script);
        scripts.value = scripts.value.toSorted((a, b) => compare(a.name, b.name));
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function removeScript() {
    if (current.value && !loading.value) {
      try {
        loading.value = true;
        if (current.value.id > 0) {
          const id = current.value.id;
          await commands.removeScript(id);
          if (current.value.id === id) {
            current.value = null;
          }

          scripts.value = scripts.value.filter((script) => {
            return script.id !== id;
          });
        }
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function importScripts() {
    if (!loading.value) {
      try {
        loading.value = true;
        const files = await dialog.open({
          directory: false,
          multiple: true,
          filters: [{ name: 'Lua', extensions: ['lua'] }],
        });

        if (Array.isArray(files) && files.length > 0) {
          const ids = await commands.importScripts(files);
          if (ids.length > 0) await loadScripts();
        }
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function exportScripts() {
    if (current.value && !loading.value) {
      try {
        loading.value = true;
        const script = current.value;
        const dir = await dialog.open({
          directory: true,
          multiple: false,
        });

        if (dir) {
          await commands.exportScript(dir, script);
        }
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
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
    loading: readonly(loading),
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

function createEmptyScript(owner: PlayerId) {
  return {
    id: 0,
    name: 'Script',
    code: '',
    owner,
  };
}
