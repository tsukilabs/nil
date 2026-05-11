// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { watch } from "vue";
import { toContinentKeyRef } from "@/composables/toRef";
import type { ContinentKey } from "@/types/core/continent";
import { PublicCityImpl } from "@/core/model/city/public-city";
import { asyncRef, type AsyncRefOptions, type MaybeNilRef } from "@tb-dev/vue";

export function usePublicCity(key?: MaybeNilRef<ContinentKey>, options?: AsyncRefOptions) {
  const keyRef = toContinentKeyRef(key);
  const { state, loading, load } = asyncRef(null, async () => {
    return keyRef.value ? PublicCityImpl.load(keyRef.value) : null;
  }, options);

  watch(keyRef, load);

  return {
    city: state,
    loading,
    load,
  };
}
