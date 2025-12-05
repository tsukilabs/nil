// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { computed, nextTick } from 'vue';
import { handleError } from '@/lib/error';
import { toPlayerRef } from '@/composables/util/toRef';
import type { PlayerImpl } from '@/core/model/player/player';
import { usePlayerTurn } from '@/composables/player/usePlayerTurn';

export function usePlayerReady(player?: MaybeNilRef<PlayerImpl>) {
  const { round } = NIL.round.refs();
  const playerRef = toPlayerRef(player);
  const isPlayerTurn = usePlayerTurn();

  const isPlayerReady = computed(() => {
    if (round.value && playerRef.value) {
      return round.value.isPlayerReady(playerRef.value.id);
    }

    return false;
  });

  async function setPlayerReady(isReady: boolean) {
    await nextTick();
    if (isPlayerTurn.value) {
      try {
        await commands.setPlayerReady(isReady);
      }
      catch (err) {
        handleError(err);
      }
    }
  }

  async function togglePlayerReady() {
    await setPlayerReady(!isPlayerReady.value);
  }

  return {
    isPlayerTurn,
    isPlayerReady,
    setPlayerReady,
    togglePlayerReady,
  };
}
