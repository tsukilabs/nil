/* eslint-disable @typescript-eslint/naming-convention */
/* eslint-disable no-inner-declarations */
/* eslint-disable no-var */
/* eslint-disable @typescript-eslint/consistent-type-definitions */
import type { App } from 'vue';
import type { go } from '@/router';
import type { commands } from '@/lib/api';
import type { Round } from '@/core/round';
import type { MaybePromise } from '@tb-dev/utils';
import type { CurrentPlayer } from '@/core/current-player';
import type { CurrentVillage } from '@/core/current-village';

declare global {
  var __APP__: App;

  var NIL: {
    readonly player: {
      readonly refs: (typeof CurrentPlayer)['refs'];
      readonly setId: (typeof CurrentPlayer)['setId'];
      readonly update: (typeof CurrentPlayer)['update'];
      readonly use: (typeof CurrentPlayer)['use'];
    };
    readonly round: {
      readonly refs: (typeof Round)['refs'];
      readonly update: (typeof Round)['update'];
      readonly use: (typeof Round)['use'];
    };
    readonly village: {
      readonly refs: (typeof CurrentVillage)['refs'];
      readonly use: (typeof CurrentVillage)['use'];
    };
  };

  interface ErrorConstructor {
    panic: (message: string) => never;
    todo: (message?: string) => never;
    unimplemented: (message?: string) => never;
  }

  interface Promise<T> {
    handleError: () => void;
  }
}

declare module 'vue' {
  interface ComponentCustomProperties {
    $c: typeof commands;
    $go: typeof go;
  }
}
