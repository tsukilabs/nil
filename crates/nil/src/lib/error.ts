import * as dialog from '@/lib/dialog';
import type { MaybePromise } from '@tb-dev/utils';

export function handleError(err: unknown) {
  console.error(err);
  if (err instanceof Error) {
    dialog.error(err.message);
  } else {
    dialog.error(String(err));
  }
}

type CatchFn<T extends object> = (this: T, target: T, err: unknown) => MaybePromise<void>;
type AcceptableCatchMethod<T extends object> = {
  [P in keyof T]: T[P] extends (this: T, err: unknown) => MaybePromise<void> ? P : never;
};
type CatchMethod<T extends object> = AcceptableCatchMethod<T>[keyof T];

type FinallyFn<T extends object> = (this: T, target: T) => MaybePromise<void>;
type AcceptableFinallyMethod<T extends object> = {
  [P in keyof T]: T[P] extends (this: T) => MaybePromise<void> ? P : never;
};
type FinallyMethod<T extends object> = AcceptableFinallyMethod<T>[keyof T];

interface HandleOptions<T extends object> {
  readonly async?: boolean;
  readonly catch?: CatchFn<T> | CatchMethod<T>;
  readonly finally?: FinallyFn<T> | FinallyMethod<T>;
}

/** **DO NOT** use this decorator on a method that returns a value. */
export function HandleError<T extends object>(options: HandleOptions<T> = {}) {
  return function (_target: T, _key: string, descriptor: PropertyDescriptor) {
    const method = descriptor.value;
    if (options.async) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      descriptor.value = async function (...args: any[]) {
        try {
          await Reflect.apply(method, this, args);
        } catch (err) {
          handleError(err);
          if (typeof options.catch === 'function') {
            await Reflect.apply(options.catch, this, [this, err]);
          } else if (typeof options.catch === 'string') {
            await Reflect.apply(Reflect.get(this, options.catch), this, [err]);
          }
        } finally {
          if (typeof options.finally === 'function') {
            await Reflect.apply(options.finally, this, [this]);
          } else if (typeof options.finally === 'string') {
            await Reflect.apply(Reflect.get(this, options.finally), this, []);
          }
        }
      };
    } else {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      descriptor.value = function (...args: any[]) {
        try {
          Reflect.apply(method, this, args);
        } catch (err) {
          handleError(err);
          if (typeof options.catch === 'function') {
            Reflect.apply(options.catch, this, [this, err]);
          } else if (typeof options.catch === 'string') {
            Reflect.apply(Reflect.get(this, options.catch), this, [err]);
          }
        } finally {
          if (typeof options.finally === 'function') {
            Reflect.apply(options.finally, this, [this]);
          } else if (typeof options.finally === 'string') {
            Reflect.apply(Reflect.get(this, options.finally), this, []);
          }
        }
      };
    }
  };
}
