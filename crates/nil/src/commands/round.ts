import { invoke } from '@tauri-apps/api/core';
import type { RoundState } from '@/types/round';

export function getRoundState() {
  return invoke<RoundState>('get_round_state');
}
