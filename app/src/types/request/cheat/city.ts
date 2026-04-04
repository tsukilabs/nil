// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface CheatSetStabilityRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly stability: number;
}
