// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toNonZeroU32 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';

export async function cheatSkipRound(amount?: Option<number>) {
  amount = toNonZeroU32(amount ?? 1);
  await invoke('cheat_skip_round', { req: { amount } });
}
