// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

/* eslint-disable no-inner-declarations */
import type { App } from 'vue';
import type { go } from '@/router';
import type { commands } from '@/lib/api';
import type { ChatEntity } from '@/core/entity/chat';
import type { RoundEntity } from '@/core/entity/round';
import type { WorldEntity } from '@/core/entity/world';
import type { MaybePromise, Option } from '@tb-dev/utils';
import type { CurrentPlayerEntity } from '@/core/entity/current-player';
import type { CurrentVillageEntity } from '@/core/entity/current-village';

declare global {
  var __APP__: App;
  var __DEBUG_ASSERTIONS__: boolean;

  var NIL: {
    readonly chat: {
      readonly refs: (typeof ChatEntity)['refs'];
      readonly update: (typeof ChatEntity)['update'];
      readonly use: (typeof ChatEntity)['use'];
    };

    /** Jogador atual. */
    readonly player: {
      readonly refs: (typeof CurrentPlayerEntity)['refs'];
      readonly setId: (typeof CurrentPlayerEntity)['setId'];
      readonly update: (typeof CurrentPlayerEntity)['update'];
      readonly use: (typeof CurrentPlayerEntity)['use'];
    };

    readonly round: {
      readonly refs: (typeof RoundEntity)['refs'];
      readonly update: (typeof RoundEntity)['update'];
      readonly use: (typeof RoundEntity)['use'];
    };

    /** Aldeia atual. */
    readonly village: {
      readonly refs: (typeof CurrentVillageEntity)['refs'];
      readonly setCoord: (typeof CurrentVillageEntity)['setCoord'];
      readonly use: (typeof CurrentVillageEntity)['use'];
    };

    readonly world: {
      readonly refs: (typeof WorldEntity)['refs'];
      readonly use: (typeof WorldEntity)['use'];
    };

    /** Atualiza todas as entidades. */
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
