// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { BuildingId } from '@/types/core/infrastructure/building';

export interface ToggleBuildingRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly id: BuildingId;
  readonly enabled: boolean;
}
