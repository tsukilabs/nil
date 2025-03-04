import type { ShallowRef } from 'vue';
import { handleError } from '@/lib/error';
import { useAsyncState } from '@vueuse/core';

export type AsyncRefOptions = {
  immediate?: boolean;
};

export function asyncRef<T>(initial: T, fn: () => Promise<T>, options?: AsyncRefOptions) {
  const { execute, state } = useAsyncState<T>(fn, initial, {
    immediate: options?.immediate ?? true,
    onError: handleError,
    resetOnExecute: false,
    shallow: true,
    throwError: false,
  });

  return Object.assign(state as Readonly<ShallowRef<T>>, { execute });
}
