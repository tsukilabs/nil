// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { App } from 'vue';
import Food from './resources/Food.vue';
import Iron from './resources/Iron.vue';
import Wood from './resources/Wood.vue';
import Stone from './resources/Stone.vue';
import Workforce from './resources/Workforce.vue';

export function registerGlobalComponents(app: App) {
  app
    .component('Food', Food)
    .component('Iron', Iron)
    .component('Stone', Stone)
    .component('Wood', Wood)
    .component('Workforce', Workforce);
}

declare module 'vue' {
  interface GlobalComponents {
    Food: typeof Food;
    Iron: typeof Iron;
    Stone: typeof Stone;
    Wood: typeof Wood;
    Workforce: typeof Workforce;
  }
}
