// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingLevel, MineId, MineStats } from '@/types/bindings';

export interface MineStatsTable {
  readonly id: MineId;
  readonly table: ReadonlyMap<BuildingLevel, MineStats>;
}
