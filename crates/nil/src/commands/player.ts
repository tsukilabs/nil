import { invoke } from '@tauri-apps/api/core';

export function getPlayer(id: PlayerId) {
  return invoke<Player>('get_player', { id });
}

export async function getPlayerVillages(id: PlayerId) {
  return invoke<readonly Coord[]>('get_player_villages', { id });
}

export function getPlayers() {
  return invoke<readonly Player[]>('get_players');
}

export function removePlayer(id: PlayerId) {
  return invoke<null>('remove_player', { id });
}

export function spawnPlayer(options: PlayerOptions) {
  return invoke<null>('spawn_player', { options });
}
