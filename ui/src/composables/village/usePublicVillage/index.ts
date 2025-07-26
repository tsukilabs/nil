// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { watch } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import { toContinentKeyRef } from '@/composables/util/toRef';
import { PublicVillageImpl } from '@/core/model/village/public-village';

export function usePublicVillage(key?: MaybeNilRef<ContinentKey>) {
  const keyRef = toContinentKeyRef(key);
  const { state, isLoading, execute } = asyncRef(null, async () => {
    return keyRef.value ? PublicVillageImpl.load(keyRef.value) : null;
  });

  watch(keyRef, execute);

  return {
    village: state,
    loading: isLoading,
  };
}
