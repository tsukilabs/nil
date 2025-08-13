// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

/* eslint-disable no-inner-declarations */

import type { App } from 'vue';
import type { go } from '@/router';
import type { commands } from '@/lib/api';
import type { ChatEntity } from '@/core/entity/chat';
import type { RoundEntity } from '@/core/entity/round';
import type { WorldEntity } from '@/core/entity/world';
import type { PlayerEntity } from '@/core/entity/player';
import type { VillageEntity } from '@/core/entity/village';
import type { MilitaryEntity } from '@/core/entity/military';

declare global {
  var __APP__: App;
  var __DEBUG_ASSERTIONS__: boolean;
  var __DESKTOP__: boolean;
  var __MOBILE__: boolean;

  var NIL: {
    readonly chat: {
      readonly refs: (typeof ChatEntity)['refs'];
      readonly update: (typeof ChatEntity)['update'];
      readonly use: (typeof ChatEntity)['use'];
    };

    readonly military: {
      readonly refs: (typeof MilitaryEntity)['refs'];
      readonly update: (typeof MilitaryEntity)['update'];
      readonly use: (typeof MilitaryEntity)['use'];
    };

    /** Current player. */
    readonly player: {
      readonly refs: (typeof PlayerEntity)['refs'];
      readonly setId: (typeof PlayerEntity)['setId'];
      readonly update: (typeof PlayerEntity)['update'];
      readonly use: (typeof PlayerEntity)['use'];
    };

    readonly round: {
      readonly refs: (typeof RoundEntity)['refs'];
      readonly update: (typeof RoundEntity)['update'];
      readonly use: (typeof RoundEntity)['use'];
    };

    /** Current village. */
    readonly village: {
      readonly refs: (typeof VillageEntity)['refs'];
      readonly setCoord: (typeof VillageEntity)['setCoord'];
      readonly use: (typeof VillageEntity)['use'];
    };

    readonly world: {
      readonly refs: (typeof WorldEntity)['refs'];
      readonly use: (typeof WorldEntity)['use'];
    };

    readonly console: typeof import('@tb-dev/vue').console;

    /** Updates **all** entities. */
    readonly update: () => Promise<void>;
  };

  interface ErrorConstructor {
    throw: (message: string) => never;
    todo: (message?: Option<string>) => never;
  }

  interface Promise<T> {
    err: (message?: Option<string>) => void;
  }
}
