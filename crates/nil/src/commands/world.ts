import { invoke } from '@tauri-apps/api/core';

export function getWorldState() {
  return invoke<WorldState>('get_world_state');
}
