import type { Ref } from 'vue';
import { handleError } from '@/lib/error';
import { type AsyncComputedOnCancel, type AsyncComputedOptions, computedAsync } from '@vueuse/core';

export function asyncComputed<T>(
  initial: T,
  callback: (onCancel: AsyncComputedOnCancel) => Promise<T> | T,
  options?: AsyncComputedOptions
) {
  return computedAsync(callback, initial, { onError: handleError, ...options }) as Readonly<Ref<T>>;
}
