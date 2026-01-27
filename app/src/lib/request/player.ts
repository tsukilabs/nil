// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetPlayerRequest {
  readonly world: WorldId;
  readonly id: PlayerId;
}

export interface GetPlayerCoordsRequest {
  readonly world: WorldId;
  readonly id: PlayerId;
}

export interface GetPlayersRequest {
  readonly world: WorldId;
}

export interface GetPlayerMaintenanceRequest {
  readonly world: WorldId;
}

export interface GetPlayerMilitaryRequest {
  readonly world: WorldId;
}

export interface GetPlayerReportsRequest {
  readonly world: WorldId;
}

export interface GetPlayerStatusRequest {
  readonly world: WorldId;
  readonly id: PlayerId;
}

export interface GetPlayerStorageCapacityRequest {
  readonly world: WorldId;
}

export interface GetPublicPlayerRequest {
  readonly world: WorldId;
  readonly id: PlayerId;
}

export interface GetPublicPlayersRequest {
  readonly world: WorldId;
}

export interface PlayerExistsRequest {
  readonly world: WorldId;
  readonly id: PlayerId;
}

export interface SetPlayerStatusRequest {
  readonly world: WorldId;
  readonly status: PlayerStatus;
}

export interface SpawnPlayerRequest {
  readonly world: WorldId;
  readonly options: PlayerOptions;
}
