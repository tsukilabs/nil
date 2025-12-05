// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getRound() {
  return invoke<Round>('get_round');
}

export async function isRoundIdle() {
  return invoke<boolean>('is_round_idle');
}

export async function isRoundWaiting() {
  return invoke<boolean>('is_round_waiting');
}

export async function setPlayerReady(isReady: boolean) {
  await invoke('set_player_ready', { req: { isReady } });
}

export async function startRound() {
  await invoke('start_round');
}
