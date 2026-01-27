// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toNonZeroU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import type { CheatSkipRoundRequest } from '@/lib/request';

export async function cheatSkipRound(amount?: Option<number>) {
  amount = toNonZeroU8(amount ?? 1);
  const req: CheatSkipRoundRequest = {
    world: NIL.world.getIdStrict(),
    amount,
  };

  await invoke('cheat_skip_round', { req });
}
