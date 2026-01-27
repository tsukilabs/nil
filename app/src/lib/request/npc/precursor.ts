// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetPrecursorCoordsRequest {
  readonly world: WorldId;
  readonly id: PrecursorId;
}

export interface GetPublicPrecursorRequest {
  readonly world: WorldId;
  readonly id: PrecursorId;
}
