// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getPlayer(id: PlayerId) {
  return invoke<Player>('get_player', { id });
}

export async function getPlayerCoords(id: PlayerId) {
  return invoke<readonly Coord[]>('get_player_coords', { id });
}

export async function getPlayerMaintenance() {
  return invoke<number>('get_player_maintenance');
}

export async function getPlayerStatus(id: PlayerId) {
  return invoke<PlayerStatus>('get_player_status', { id });
}

export async function getPlayerStorageCapacity() {
  return invoke<PlayerStorageCapacity>('get_player_storage_capacity');
}

export function getPlayers() {
  return invoke<readonly Player[]>('get_players');
}

export function playerExists(id: PlayerId) {
  return invoke<boolean>('player_exists', { id });
}

export function setPlayerStatus(status: PlayerStatus) {
  return invoke<null>('set_player_status', { status });
}

export function spawnPlayer(options: PlayerOptions) {
  return invoke<null>('spawn_player', { options });
}
