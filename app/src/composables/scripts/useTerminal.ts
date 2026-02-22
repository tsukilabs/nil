// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { markRaw, readonly, ref } from 'vue';
import { sessionRef, useMutex } from '@tb-dev/vue';
import { ScriptOutputImpl } from '@/core/model/scripts/script-output';

type LineKind = 'input' | 'output';

class Line {
  private static id = 0;

  public readonly id: number;
  public readonly time: number;

  constructor(
    public readonly kind: LineKind,
    public readonly content: string,
    time?: Option<number>,
  ) {
    this.id = ++Line.id;
    this.time = time ?? Date.now();
  }
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

  function pushLine(kind: LineKind, content: string, time?: number) {
    const push = (value: string) => {
      lines.value.push(markRaw(new Line(kind, value, time)));
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
