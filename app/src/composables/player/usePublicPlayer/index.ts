// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { asyncRef } from '@tb-dev/vue';
import { computed, toRef, watch } from 'vue';
import { PublicPlayerImpl } from '@/core/model/player/public-player';

export function usePublicPlayer(id: MaybeNilRef<PlayerId>) {
  const idRef = toRef(id);
  const { state, isLoading, execute } = asyncRef(null, async () => {
    return idRef.value ? PublicPlayerImpl.load(idRef.value) : null;
  });

  const coords = computed(() => state.value?.coords ?? []);

  watch(idRef, execute);

  return {
    player: state,
    coords,
    loading: isLoading,
    load: execute,
  };
}
