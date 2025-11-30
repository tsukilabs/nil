// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function endTurn() {
  return invoke<null>('end_turn');
}

export function getRound() {
  return invoke<Round>('get_round');
}

export function isRoundIdle() {
  return invoke<boolean>('is_round_idle');
}

export function startRound() {
  return invoke<null>('start_round');
}
