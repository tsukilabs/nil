// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toNonZeroU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';

export async function cheatSkipRound(amount?: Option<number>) {
  amount = toNonZeroU8(amount ?? 1);
  await invoke('cheat_skip_round', { req: { amount } });
}
