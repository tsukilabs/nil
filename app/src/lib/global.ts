// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { console } from '@tb-dev/vue';
import { camelCase } from 'es-toolkit';
import * as commands from '@/commands';
import * as cheats from '@/commands/cheat';

export function defineGlobalCommands() {
  if (__DEBUG_ASSERTIONS__ && !Object.hasOwn(globalThis.NIL, 'cmd')) {
    Object.defineProperty(globalThis.NIL, 'cmd', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: commands,
    });
  }
}

export function defineGlobalCheats() {
  if (!Object.hasOwn(globalThis.NIL, 'cheat')) {
    const regex = /^cheat/;
    const value = Array.from(Object.entries(cheats))
      .filter(([key, _]) => key.startsWith('cheat'))
      .filter(([_, fn]) => typeof fn === 'function')
      .map(([key, fn]) => {
        key = camelCase(key.replace(regex, ''));
        return [key, fn];
      });

    Object.defineProperty(globalThis.NIL, 'cheat', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: Object.fromEntries(value),
    });
  }
}

export function defineReactiveConsole() {
  if (__DEBUG_ASSERTIONS__ && !Object.hasOwn(globalThis.NIL, 'console')) {
    Object.defineProperty(globalThis.NIL, 'console', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: console,
    });
  }
}
