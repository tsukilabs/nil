// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { shallowRef } from 'vue';
import * as commands from '@/commands';
import * as dialog from '@/lib/dialog';
import { asyncComputed, useMutex } from '@tb-dev/vue';

export interface ScriptContents {
  readonly readme: string;
  readonly script: string;
}

export function useNsr() {
  const registry = shallowRef<readonly NsrScript[]>([]);
  const current = shallowRef<Option<NsrScript>>();
  const { locked, lock } = useMutex();

  const cache = new Map<string, ScriptContents>();

  const contents = asyncComputed<Option<ScriptContents>>(null, async () => {
    const id = current.value?.id;
    if (id) {
      let cached = cache.get(id);
      if (!cached) {
        await lock(async () => {
          const [readme, script] = await Promise.all([
            commands.fetchNsrReadme(id),
            commands.fetchNsrScript(id),
          ]);

          cached = { readme, script };
          cache.set(id, cached);
        });
      }

      return cached;
    }

    return null;
  });

  async function loadRegistry() {
    await lock(async () => {
      registry.value = await commands.fetchNsrRegistry();
    });
  }

  async function execute() {
    await lock(async () => {
      if (contents.value) {
        await commands.executeScriptChunk(contents.value.script);
      }
    });
  }

  async function save() {
    await lock(async () => {
      const { player } = NIL.player.refs();
      if (current.value && contents.value && player.value) {
        await commands.addScript({
          name: current.value.frontmatter.name,
          code: contents.value.script,
          owner: player.value.id,
        });
      }
    });
  }

  async function download() {
    await lock(async () => {
      if (current.value && contents.value) {
        const name = current.value.frontmatter.name;
        const code = contents.value.script;
        const dir = await dialog.open({
          directory: true,
          multiple: false,
        });

        if (dir) {
          await commands.exportScript(dir, name, code);
        }
      }
    });
  }

  async function reload() {
    const id = current.value?.id;
    if (id) {
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
    loading: locked,
    loadRegistry,
    setCurrent,
    execute,
    save,
    download,
    reload,
  };
}
