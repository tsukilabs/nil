import type { Coord } from '@/types/village';
import { invoke } from '@tauri-apps/api/core';
import type { Player, PlayerId, PlayerOptions } from '@/types/player';

export function getPlayer(id: PlayerId) {
  return invoke<Player>('get_player', { id });
}

export async function getPlayerVillages(id: PlayerId) {
  return invoke<Coord[]>('get_player_villages', { id });
}

export function spawnPlayer(options: PlayerOptions) {
  return invoke<null>('spawn_player', { options });
}
