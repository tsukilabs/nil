import type { Coord } from '@/types/world';
import { invoke } from '@tauri-apps/api/core';
import type { Player, PlayerConfig, PlayerId } from '@/types/player';

export function getPlayer(id: PlayerId) {
  return invoke<Player>('get_player', { id });
}

export async function getPlayerVillages(id: PlayerId) {
  return invoke<Coord[]>('get_player_villages', { id });
}

export function spawnPlayer(config: PlayerConfig) {
  return invoke<PlayerId>('spawn_player', { config });
}
