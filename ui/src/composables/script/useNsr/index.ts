// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import * as dialog from '@/lib/dialog';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import { asyncComputed } from '@tb-dev/vue';
import { nextTick, readonly, ref, shallowRef } from 'vue';

export interface ScriptContents {
  readonly readme: string;
  readonly script: string;
}

export function useNsr() {
  const registry = shallowRef<readonly NsrScript[]>([]);
  const current = shallowRef<Option<NsrScript>>();
  const loading = ref(false);

  const cache = new Map<string, ScriptContents>();

  const contents = asyncComputed(null, async () => {
    const id = current.value?.id;
    if (id) {
      let cached = cache.get(id);
      if (!cached && !loading.value) {
        try {
          loading.value = true;
          const [readme, script] = await Promise.all([
            commands.fetchNsrReadme(id),
            commands.fetchNsrScript(id),
          ]);

          cached = { readme, script };
          cache.set(id, cached);
        } catch (err) {
          handleError(err);
        } finally {
          loading.value = false;
        }
      }

      return cached;
    }

    return null;
  });

  async function loadRegistry() {
    if (!loading.value) {
      try {
        loading.value = true;
        registry.value = await commands.fetchNsrRegistry();
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function execute() {
    await nextTick();
    if (contents.value && !loading.value) {
      try {
        loading.value = true;
        await commands.executeScriptChunk(contents.value.script);
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function save() {
    await nextTick();
    const { player } = NIL.player.refs();
    if (current.value && contents.value && player.value && !loading.value) {
      try {
        loading.value = true;
        await commands.addScript({
          id: 0,
          name: current.value.frontmatter.name,
          owner: player.value.id,
          code: contents.value.script,
        });
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function download() {
    if (current.value && contents.value && !loading.value) {
      try {
        loading.value = true;
        const name = current.value.frontmatter.name;
        const code = contents.value.script;
        const dir = await dialog.open({
          directory: true,
          multiple: false,
        });

        if (dir) {
          await commands.exportScript(dir, name, code);
        }
      } catch (err) {
        handleError(err);
      } finally {
        loading.value = false;
      }
    }
  }

  async function reload() {
    const id = current.value?.id;
    if (id && !loading.value) {
      cache.clear();
      await loadRegistry();
      current.value = registry.value.find((it) => it.id === id);
    }
  }

  function setCurrent(entry: NsrScript) {
    current.value = entry;
  }

  return {
    registry: registry as Readonly<typeof registry>,
    current: current as Readonly<typeof current>,
    contents,
    loading: readonly(loading),
    loadRegistry,
    setCurrent,
    execute,
    save,
    download,
    reload,
  };
}
