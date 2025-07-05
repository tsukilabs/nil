// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { camelCase } from 'es-toolkit';
import * as commands from '@/commands';
import * as cheats from '@/commands/cheat';

export async function defineGlobalCommands() {
  if (!Object.hasOwn(window, '$c') && (await commands.isDev())) {
    Object.defineProperty(window, '$c', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: commands,
    });
  }
}

export function defineGlobalCheats() {
  if (!Object.hasOwn(window.NIL, 'cheat')) {
    const { config } = NIL.world.refs();
    if (config.value?.allowCheats) {
      const regex = /^cheat/;
      const value = Array.from(Object.entries(cheats))
        .filter(([key, _]) => key.startsWith('cheat'))
        .filter(([_, fn]) => typeof fn === 'function')
        .map(([key, fn]) => {
          key = camelCase(key.replace(regex, ''));
          return [key, fn];
        });

      Object.defineProperty(window.NIL, 'cheat', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: Object.fromEntries(value),
      });
    }
  }
}
