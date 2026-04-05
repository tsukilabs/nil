// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { PlayerId, PlayerOptions, PlayerStatus } from '@/types/core/player';

export interface GetPlayerRequest {
  readonly world: WorldId;
}

export interface GetPlayerCoordsRequest {
  readonly world: WorldId;
  readonly id: PlayerId;
}

export interface GetPlayerIdsRequest {
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

export interface GetPlayerWorldsRequest {
  readonly id: PlayerId;
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
  readonly worldPassword: Option<string>;
  readonly options: PlayerOptions;
}
