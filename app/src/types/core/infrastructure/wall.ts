// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingLevel, WallStats } from '@/types/bindings';

export interface WallStatsTable {
  readonly table: ReadonlyMap<BuildingLevel, WallStats>;
}
