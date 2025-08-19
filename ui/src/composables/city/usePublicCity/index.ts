// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { watch } from 'vue';
import { asyncRef, type AsyncRefOptions } from '@tb-dev/vue';
import { toContinentKeyRef } from '@/composables/util/toRef';
import { PublicCityImpl } from '@/core/model/city/public-city';

export function usePublicCity(key?: MaybeNilRef<ContinentKey>, options?: AsyncRefOptions) {
  const keyRef = toContinentKeyRef(key);
  const { state, isLoading, execute } = asyncRef(null, async () => {
    return keyRef.value ? PublicCityImpl.load(keyRef.value) : null;
  }, options);

  watch(keyRef, execute);

  return {
    city: state,
    loading: isLoading,
    load: execute,
  };
}
