// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

/* eslint-disable no-inner-declarations */

import type { App } from 'vue';
import type { go } from '@/router';
import type { commands } from '@/lib/api';
import type { ChatEntity } from '@/core/entity/chat';
import type { CityEntity } from '@/core/entity/city';
import type { RoundEntity } from '@/core/entity/round';
import type { WorldEntity } from '@/core/entity/world';
import type { PlayerEntity } from '@/core/entity/player';
import type { MilitaryEntity } from '@/core/entity/military';

declare global {
  var __APP__: App;
  var __DEBUG_ASSERTIONS__: boolean;
  var __DESKTOP__: boolean;
  var __MOBILE__: boolean;

  var __CONSTS__: {
    readonly i16Min: number;
    readonly i16Max: number;
    readonly u8Max: number;
    readonly u16Max: number;
    readonly u32Max: number;
  };

  var NIL: {
    readonly chat: {
      readonly refs: (typeof ChatEntity)['refs'];
      readonly update: (typeof ChatEntity)['update'];
      readonly use: (typeof ChatEntity)['use'];
    };

    /** Current city. */
    readonly city: {
      readonly getCity: (typeof CityEntity)['getCity'];
      readonly getCoord: (typeof CityEntity)['getCoord'];
      readonly getProduction: (typeof CityEntity)['getProduction'];
      readonly refs: (typeof CityEntity)['refs'];
      readonly setCoord: (typeof CityEntity)['setCoord'];
      readonly use: (typeof CityEntity)['use'];
    };

    readonly military: {
      readonly refs: (typeof MilitaryEntity)['refs'];
      readonly update: (typeof MilitaryEntity)['update'];
      readonly use: (typeof MilitaryEntity)['use'];
    };

    /** Current player. */
    readonly player: {
      readonly getCoords: (typeof PlayerEntity)['getCoords'];
      readonly getId: (typeof PlayerEntity)['getId'];
      readonly getPlayer: (typeof PlayerEntity)['getPlayer'];
      readonly refs: (typeof PlayerEntity)['refs'];
      readonly setId: (typeof PlayerEntity)['setId'];
      readonly update: (typeof PlayerEntity)['update'];
      readonly use: (typeof PlayerEntity)['use'];
    };

    readonly round: {
      readonly getRound: (typeof RoundEntity)['getRound'];
      readonly refs: (typeof RoundEntity)['refs'];
      readonly update: (typeof RoundEntity)['update'];
      readonly use: (typeof RoundEntity)['use'];
    };

    readonly world: {
      readonly getConfig: (typeof WorldEntity)['getConfig'];
      readonly getContinentSize: (typeof WorldEntity)['getContinentSize'];
      readonly getStats: (typeof WorldEntity)['getStats'];
      readonly refs: (typeof WorldEntity)['refs'];
      readonly use: (typeof WorldEntity)['use'];
    };

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
