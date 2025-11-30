// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function endTurn() {
  await invoke('end_turn');
}

export async function getRound() {
  return invoke<Round>('get_round');
}

export async function isRoundIdle() {
  return invoke<boolean>('is_round_idle');
}

export async function startRound() {
  await invoke('start_round');
}
