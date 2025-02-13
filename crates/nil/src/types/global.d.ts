/* eslint-disable @typescript-eslint/consistent-type-definitions */
import type { App } from 'vue';
import type { go } from '@/router';
import type { commands } from '@/lib/api';
import type { MaybePromise } from '@tb-dev/utils';

type GlobalNil = {
  readonly app: App;
};

declare global {
  interface Window {
    readonly __NIL__: GlobalNil;
  }

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
