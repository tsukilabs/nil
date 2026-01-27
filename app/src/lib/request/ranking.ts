// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetRankingRequest {
  readonly world: WorldId;
}

export interface GetRankRequest {
  readonly world: WorldId;
  readonly ruler: Ruler;
}
