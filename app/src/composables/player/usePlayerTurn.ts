// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import type { RoundImpl } from '@/core/model/round';
import { toPlayerRef } from '@/composables/util/toRef';
import type { PlayerImpl } from '@/core/model/player/player';

export function usePlayerTurn(player?: MaybeNilRef<PlayerImpl>) {
  const { round } = NIL.round.refs();
  const playerRef = toPlayerRef(player);

  return computed(() => {
    if (round.value && playerRef.value) {
      return isPlayerTurn(round.value, playerRef.value);
    }

    return false;
  });
}

export function isPlayerTurn(round?: Option<RoundImpl>, player?: Option<Player>) {
  round ??= NIL.round.getRound();
  player ??= NIL.player.getPlayer();

  if (round && player?.id) {
    return round.isWaitingPlayer(player.id);
  }
  else {
    return false;
  }
}
