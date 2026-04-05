// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';

export interface CheatGetBuildStepsRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}
