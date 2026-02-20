// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { readonly, ref } from 'vue';
import * as commands from '@/commands';
import { sessionRef, useMutex } from '@tb-dev/vue';
import { ScriptOutputImpl } from '@/core/model/scripts/script-output';

export interface Line {
  readonly id: number;
  readonly kind: 'input' | 'output';
  readonly content: string;
  readonly time: number;
}

export function useTerminal() {
  const chunk = ref<Option<string>>();
  const { locked, ...mutex } = useMutex();

  const currentLineId = ref(0);
  const lines = sessionRef<Line[]>('terminal:lines', [], {
    deep: true,
    listenToStorageChanges: true,
    shallow: false,
  });

  async function execute() {
    if (chunk.value) {
      pushLine('input', chunk.value);

      try {
        await mutex.acquire();
        const output = await commands.executeScript(chunk.value);
        for (const { content, date } of ScriptOutputImpl.toSortedArray(output)) {
          pushLine('output', content, date.getTime());
        }
      }
      catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        pushLine('output', message);
      }
      finally {
        chunk.value = null;
        mutex.release();
      }
    }
  }

  function pushLine(kind: Line['kind'], content: string, time?: number) {
    const push = (value: string) => {
      lines.value.push({
        id: ++currentLineId.value,
        kind,
        content: value,
        time: time ?? Date.now(),
      });
    };

    if (kind === 'input') {
      push(content.trimStart());
    }
    else {
      for (const line of content.split('\n')) {
        if (line.trim().length > 0) {
          push(line.trimEnd());
        }
      }
    }
  }

  async function clear() {
    await mutex.acquire();
    currentLineId.value = 0;
    lines.value = [];
    chunk.value = '';
    mutex.release();
  }

  return {
    currentLineId: readonly(currentLineId),
    lines: lines as Readonly<typeof lines>,
    chunk,
    loading: locked,
    execute,
    clear,
  };
}
