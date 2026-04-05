// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { asyncRef } from '@tb-dev/vue';
import type { PlayerId } from '@/types/core/player';
import { computed, type MaybeRefOrGetter, toRef } from 'vue';

interface UsePlayerIdsOptions {
  exclude?: MaybeRefOrGetter<readonly PlayerId[]>;
  filter?: (playerId: PlayerId) => boolean;
  search?: MaybeRefOrGetter<Option<string>>;
}

export function usePlayerIds(options?: UsePlayerIdsOptions) {
  const { state, loading, load } = asyncRef([], async () => {
    const ids = await commands.getPlayerIds();
    return options?.filter ? ids.filter(options.filter) : ids;
  });

  const exclude = options?.exclude ? toRef(options.exclude) : null;
  const search = options?.search ? toRef(options.search) : null;

  const playerIds = computed(() => {
    let ids = state.value;

    if (Array.isArray(exclude?.value) && exclude.value.length > 0) {
      ids = ids.filter((id) => {
        return !exclude.value.includes(id);
      });
    }

    let searchValue = search?.value?.trim();
    if (searchValue) {
      searchValue = searchValue.toLocaleLowerCase();
      ids = state.value.filter((id) => {
        return id.toLocaleLowerCase().includes(searchValue!);
      });
    }

    return ids;
  });

  return {
    playerIds,
    loading,
    load,
  };
}
