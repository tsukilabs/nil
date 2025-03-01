import { invoke } from '@tauri-apps/api/core';

export function getRoundState() {
  return invoke<RoundState>('get_round_state');
}
