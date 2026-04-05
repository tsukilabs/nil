// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Army } from '@/types/core/military/army';
import type { Maneuver, ManeuverId } from '@/types/core/military/maneuver';
import type { ContinentIndex, ContinentSize } from '@/types/core/continent';

export interface Military {
  readonly continent: ReadonlyMap<ContinentIndex, readonly Army[]>;
  readonly continentSize: ContinentSize;
  readonly maneuvers: ReadonlyMap<ManeuverId, Maneuver>;
}
