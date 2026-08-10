// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { handleError } from "@/lib/error";
import { prependKey } from "@/lib/storage";
import type { Option } from "@tb-dev/utils";
import { readonly, ref, type Ref } from "vue";
import type { WorldId } from "@tsukilabs/nil-bindings";
import { useSessionStorage, type UseStorageOptions } from "@vueuse/core";

export function wref<T>(
  key: string,
  defaultValue: T,
  fn?: Option<() => Promise<T>>,
  options?: WorldRefOptions<T>,
) {
  key = prependKey(key);

  const { worldId } = NIL.world.refs();
  const state = useSessionStorage(key, defaultValue, {
    deep: options?.deep ?? true,
    initOnMounted: options?.initOnMounted ?? true,
    listenToStorageChanges: options?.listenToStorageChanges ?? true,
    mergeDefaults: options?.mergeDefaults ?? true,
    onError: options?.onError ?? handleError,
    shallow: options?.shallow ?? false,
    writeDefaults: options?.writeDefaults ?? true,
    serializer: {
      read: (raw: string) => {
        const { world, data }: WorldRefData<T> = JSON.parse(raw);
        return world === worldId.value ? data : defaultValue;
      },
      write: (data: Option<T>) => {
        return JSON.stringify({ world: worldId.value, data });
      },
    },

    ...options,
  });

  const id = ref(0);
  const loading = ref(false);

  const load = async () => {
    if (typeof fn !== "function") {
      return;
    }

    const currentId = ++id.value;
    try {
      loading.value = true;
      const data = await fn();
      if (currentId === id.value) {
        state.value = data;
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      if (currentId === id.value) {
        loading.value = false;
      }
    }
  };

  void load();

  return {
    state: state as Ref<T>,
    loading: readonly(loading),
    load,
  };
}

export type WorldRefOptions<T> = UseStorageOptions<T>;

export interface WorldRefData<T> {
  world: WorldId;
  data: T;
}
