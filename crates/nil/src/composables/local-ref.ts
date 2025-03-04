import { computed, type Ref } from 'vue';
import { handleError } from '@/lib/error';
import { useLocalStorage } from '@vueuse/core';

type Value<T> = {
  inner: T;
};

export type LocalRefOptions = {
  deep?: boolean;
  initOnMounted?: boolean;
};

export function localRef<T>(key: string, initial: T, options?: LocalRefOptions): Ref<T> {
  const defaultValue: Value<T> = { inner: initial };
  const local = useLocalStorage<Value<T>>(key, defaultValue, {
    deep: options?.deep ?? true,
    initOnMounted: options?.initOnMounted ?? true,
    onError: handleError,
    writeDefaults: true,

    serializer: {
      read: JSON.parse,
      write: JSON.stringify,
    },
  });

  return computed<T>({
    get: () => local.value.inner,
    set: (value: T) => {
      local.value.inner = value;
    },
  });
}
