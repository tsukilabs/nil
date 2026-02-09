// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toRef } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import { tryOnScopeDispose } from '@vueuse/core';
import type { SavedataFile } from '@/core/savedata';
import { getSavedataPlayers } from '@/commands/world';

export function useSavedataPlayers(savedata: MaybeNilRef<SavedataFile>) {
  const savedataRef = toRef(savedata);
  const cache = new Map<string, readonly PlayerId[]>();

  const { state, loading, load } = asyncRef<readonly PlayerId[]>([], async () => {
    let players: readonly PlayerId[] = [];
    if (savedataRef.value) {
      players = cache.get(savedataRef.value.path) ?? [];
      if (players.length === 0) {
        players = await getSavedataPlayers(savedataRef.value.path);
        cache.set(savedataRef.value.path, players);
      }
    }

    return players;
  });

  tryOnScopeDispose(() => cache.clear());

  return {
    players: state as Readonly<typeof state>,
    loading,
    load,
  };
}
