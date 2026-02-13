// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface CreateRemoteWorldRequest {
  readonly options: WorldOptions;
  readonly description: Option<string>;
  readonly password: Option<string>;
}

export interface GetRemoteWorldRequest {
  readonly world: WorldId;
}

export interface GetWorldBotsRequest {
  readonly world: WorldId;
}

export interface GetWorldConfigRequest {
  readonly world: WorldId;
}

export interface GetWorldPlayersRequest {
  readonly world: WorldId;
}

export interface GetWorldPrecursorsRequest {
  readonly world: WorldId;
}

export interface GetWorldStatsRequest {
  readonly world: WorldId;
}

export interface SaveLocalWorldRequest {
  readonly world: WorldId;
  readonly path: string;
}
