// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { RawMilitary } from '@/core/model/military/military';
import type {
  GetPlayerCoordsRequest,
  GetPlayerMaintenanceRequest,
  GetPlayerMilitaryRequest,
  GetPlayerReportsRequest,
  GetPlayerRequest,
  GetPlayersRequest,
  GetPlayerStatusRequest,
  GetPlayerStorageCapacityRequest,
  GetPublicPlayerRequest,
  GetPublicPlayersRequest,
  PlayerExistsRequest,
  SetPlayerStatusRequest,
  SpawnPlayerRequest,
} from '@/lib/request';

export async function getPlayer(id: PlayerId) {
  const req: GetPlayerRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<Player>('get_player', { req });
}

export async function getPlayerCoords(id: PlayerId) {
  const req: GetPlayerCoordsRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<readonly Coord[]>('get_player_coords', { req });
}

export async function getPlayerMaintenance() {
  const req: GetPlayerMaintenanceRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<number>('get_player_maintenance', { req });
}

export async function getPlayerMilitary() {
  const req: GetPlayerMilitaryRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<RawMilitary>('get_player_military', { req });
}

export async function getPlayerReports() {
  const req: GetPlayerReportsRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<ReportId[]>('get_player_reports', { req });
}

export async function getPlayerStatus(id: PlayerId) {
  const req: GetPlayerStatusRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<PlayerStatus>('get_player_status', { req });
}

export async function getPlayerStorageCapacity() {
  const req: GetPlayerStorageCapacityRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<OverallStorageCapacity>('get_player_storage_capacity', { req });
}

export async function getPlayers() {
  const req: GetPlayersRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<readonly Player[]>('get_players', { req });
}

export async function getPublicPlayer(id: PlayerId) {
  const req: GetPublicPlayerRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<PublicPlayer>('get_public_player', { req });
}

export async function getPublicPlayers() {
  const req: GetPublicPlayersRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<readonly PublicPlayer[]>('get_public_players', { req });
}

export async function playerExists(id: PlayerId) {
  const req: PlayerExistsRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<boolean>('player_exists', { req });
}

export async function setPlayerStatus(status: PlayerStatus) {
  const req: SetPlayerStatusRequest = {
    world: NIL.world.getIdStrict(),
    status,
  };

  await invoke('set_player_status', { req });
}

export async function spawnPlayer(options: PlayerOptions) {
  const req: SpawnPlayerRequest = {
    world: NIL.world.getIdStrict(),
    options,
  };

  await invoke('spawn_player', { req });
}
