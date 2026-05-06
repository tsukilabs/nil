// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';
import type {
  GetPlayerCoordsRequest,
  GetPlayerCoordsResponse,
  GetPlayerIdsRequest,
  GetPlayerIdsResponse,
  GetPlayerMaintenanceRequest,
  GetPlayerMaintenanceResponse,
  GetPlayerMilitaryRequest,
  GetPlayerMilitaryResponse,
  GetPlayerReportsRequest,
  GetPlayerReportsResponse,
  GetPlayerRequest,
  GetPlayerResponse,
  GetPlayerStatusRequest,
  GetPlayerStatusResponse,
  GetPlayerStorageCapacityRequest,
  GetPlayerStorageCapacityResponse,
  GetPlayerWorldsRequest,
  GetPlayerWorldsResponse,
  GetPublicPlayerRequest,
  GetPublicPlayerResponse,
  GetPublicPlayersRequest,
  GetPublicPlayersResponse,
  PlayerExistsRequest,
  PlayerExistsResponse,
  PlayerId,
  PlayerOptions,
  PlayerStatus,
  SetPlayerStatusRequest,
  SpawnPlayerRequest,
} from '@/types/bindings';

export async function getPlayer() {
  const req: GetPlayerRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPlayerResponse>('get_player', { req });
}

export async function getPlayerCoords(id: PlayerId) {
  const req: GetPlayerCoordsRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetPlayerCoordsResponse>('get_player_coords', { req });
}

export async function getPlayerIds() {
  const req: GetPlayerIdsRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPlayerIdsResponse>('get_player_ids', { req });
}

export async function getPlayerMaintenance() {
  const req: GetPlayerMaintenanceRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPlayerMaintenanceResponse>('get_player_maintenance', { req });
}

export async function getPlayerMilitary() {
  const req: GetPlayerMilitaryRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPlayerMilitaryResponse>('get_player_military', { req });
}

export async function getPlayerReports() {
  const req: GetPlayerReportsRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPlayerReportsResponse>('get_player_reports', { req });
}

export async function getPlayerStatus(id: PlayerId) {
  const req: GetPlayerStatusRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetPlayerStatusResponse>('get_player_status', { req });
}

export async function getPlayerStorageCapacity() {
  const req: GetPlayerStorageCapacityRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPlayerStorageCapacityResponse>('get_player_storage_capacity', { req });
}

export async function getPlayerWorlds(id: PlayerId) {
  const req: GetPlayerWorldsRequest = { id };
  return invoke<GetPlayerWorldsResponse>('get_player_worlds', { req });
}

export async function getPublicPlayer(id: PlayerId) {
  const req: GetPublicPlayerRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetPublicPlayerResponse>('get_public_player', { req });
}

export async function getPublicPlayers() {
  const req: GetPublicPlayersRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPublicPlayersResponse>('get_public_players', { req });
}

export async function playerExists(id: PlayerId) {
  const req: PlayerExistsRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<PlayerExistsResponse>('player_exists', { req });
}

export async function setPlayerStatus(status: PlayerStatus) {
  const req: SetPlayerStatusRequest = {
    world: NIL.world.getIdStrict(),
    status,
  };

  await invoke('set_player_status', { req });
}

export async function spawnPlayer(options: PlayerOptions, worldPassword: Option<string>) {
  const req: SpawnPlayerRequest = {
    world: NIL.world.getIdStrict(),
    worldPassword: worldPassword ?? null,
    options,
  };

  await invoke('spawn_player', { req });
}
