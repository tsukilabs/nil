// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import type { PlayerId } from '@/types/core/player';
import { computed, type MaybeRefOrGetter, toRef } from 'vue';

interface UsePlayerIdsOptions {
  exclude?: MaybeRefOrGetter<readonly PlayerId[]>;
  filter?: (playerId: PlayerId) => boolean;
}

export function usePlayerIds(options?: UsePlayerIdsOptions) {
  const { state, loading, load } = asyncRef([], async () => {
    const ids = await commands.getPlayerIds();
    return options?.filter ? ids.filter(options.filter) : ids;
  });

  const exclude = options?.exclude ? toRef(options.exclude) : null;

  const playerIds = computed(() => {
    if (Array.isArray(exclude?.value) && exclude.value.length > 0) {
      return state.value.filter((id) => !exclude.value.includes(id));
    }
    else {
      return state.value;
    }
  });

  return {
    playerIds,
    loading,
    load,
  };
}
