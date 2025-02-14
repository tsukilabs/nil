import type { Ref } from 'vue';
import { handleError } from '@/lib/error';
import { useAsyncState, type UseAsyncStateOptions } from '@vueuse/core';

export function asyncRef<T>(
  initial: T,
  fn: () => Promise<T>,
  options?: UseAsyncStateOptions<true>
) {
  const { execute, state } = useAsyncState<T>(fn, initial, {
    immediate: true,
    onError: handleError,
    resetOnExecute: false,
    shallow: true,
    throwError: false,
    ...options,
  });

  return Object.assign(state as Readonly<Ref<T>>, { execute });
}
