// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';

export interface CheatSetStabilityRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly stability: number;
}
