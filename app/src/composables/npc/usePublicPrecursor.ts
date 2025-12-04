// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { asyncRef } from '@tb-dev/vue';
import { computed, toRef, watch } from 'vue';
import { PublicPrecursorImpl } from '@/core/model/npc/public-precursor';

export function usePublicPrecursor(id: MaybeNilRef<PrecursorId>) {
  const idRef = toRef(id);
  const { state, isLoading, execute } = asyncRef(null, async () => {
    return idRef.value ? PublicPrecursorImpl.load(idRef.value) : null;
  });

  const coords = computed(() => state.value?.coords ?? []);

  watch(idRef, execute);

  return {
    precursor: state,
    coords,
    loading: isLoading,
    load: execute,
  };
}
