// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { App } from 'vue';
import { camelCase } from 'es-toolkit';
import * as commands from '@/commands';
import * as cheats from '@/commands/cheat';
import Food from '@/components/resources/Food.vue';
import Iron from '@/components/resources/Iron.vue';
import Wood from '@/components/resources/Wood.vue';
import Stone from '@/components/resources/Stone.vue';
import Workforce from '@/components/resources/Workforce.vue';

export const DESKTOP = globalThis.__DESKTOP__;
export const MOBILE = globalThis.__MOBILE__;

declare module 'vue' {
  interface GlobalComponents {
    Food: typeof Food;
    Iron: typeof Iron;
    Stone: typeof Stone;
    Wood: typeof Wood;
    Workforce: typeof Workforce;
  }
}

export function registerGlobalComponents(app: App) {
  app
    .component('Food', Food)
    .component('Iron', Iron)
    .component('Stone', Stone)
    .component('Wood', Wood)
    .component('Workforce', Workforce);
}

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
  if (__DEBUG_ASSERTIONS__ && !Object.hasOwn(globalThis.NIL, 'cheat')) {
    const regex = /^cheat/;
    const entries = Array.from(Object.entries(cheats))
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
      value: Object.fromEntries(entries),
    });
  }
}
