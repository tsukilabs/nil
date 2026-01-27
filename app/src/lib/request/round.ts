// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetRoundRequest {
  readonly world: WorldId;
}

export interface SetPlayerReadyRequest {
  readonly world: WorldId;
  readonly isReady: boolean;
}

export interface StartRoundRequest {
  readonly world: WorldId;
}
