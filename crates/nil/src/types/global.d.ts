import type { App } from 'vue';
import type { go } from '@/router';
import type { commands } from '@/lib/api';
import type { MaybePromise } from '@tb-dev/utils';

interface GlobalNil {
  readonly app: App;
}

declare global {
  interface Window {
    readonly __NIL__: GlobalNil;
  }

  interface ErrorConstructor {
    throw: (message: string) => never;
  }

  interface Promise<T> {
    handleError: (onfinally?: () => MaybePromise<unknown>) => void;
  }

  interface PromiseConstructor {
    try: <T>(fn: () => MaybePromise<T>) => Promise<Awaited<T>>;
  }
}

declare module 'vue' {
  interface ComponentCustomProperties {
    $c: typeof commands;
    $go: typeof go;
  }
}
