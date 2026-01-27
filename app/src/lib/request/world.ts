// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetWorldConfigRequest {
  readonly world: WorldId;
}

export interface GetWorldStatsRequest {
  readonly world: WorldId;
}

export interface SaveWorldRequest {
  readonly world: WorldId;
  readonly path: string;
}
