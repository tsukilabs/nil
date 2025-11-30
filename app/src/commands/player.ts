// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { RawMilitary } from '@/core/model/military/military';

export async function getPlayer(id: PlayerId) {
  return invoke<Player>('get_player', { req: { id } });
}

export async function getPlayerCoords(id: PlayerId) {
  return invoke<readonly Coord[]>('get_player_coords', { id });
}

export async function getPlayerMaintenance() {
  return invoke<number>('get_player_maintenance');
}

export async function getPlayerMilitary() {
  return invoke<RawMilitary>('get_player_military');
}

export async function getPlayerStatus(id: PlayerId) {
  return invoke<PlayerStatus>('get_player_status', { id });
}

export async function getPlayerStorageCapacity() {
  return invoke<OverallStorageCapacity>('get_player_storage_capacity');
}

export async function getPlayers() {
  return invoke<readonly Player[]>('get_players');
}

export async function getPublicPlayer(id: PlayerId) {
  return invoke<PublicPlayer>('get_public_player', { id });
}

export async function getPublicPlayers() {
  return invoke<readonly PublicPlayer[]>('get_public_players');
}

export async function playerExists(id: PlayerId) {
  return invoke<boolean>('player_exists', { id });
}

export async function setPlayerStatus(status: PlayerStatus) {
  await invoke('set_player_status', { req: { status } });
}

export async function spawnPlayer(options: PlayerOptions) {
  await invoke('spawn_player', { req: { options } });
}
