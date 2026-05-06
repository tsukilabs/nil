// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingId, MineId, StorageId } from '@/types/bindings';
import type { MineStatsTable } from '@/types/core/infrastructure/mine';
import type { WallStatsTable } from '@/types/core/infrastructure/wall';
import type { StorageStatsTable } from '@/types/core/infrastructure/storage';
import type { BuildingStatsTable } from '@/types/core/infrastructure/building';

export interface InfrastructureStats {
  readonly building: ReadonlyMap<BuildingId, BuildingStatsTable>;
  readonly mine: ReadonlyMap<MineId, MineStatsTable>;
  readonly storage: ReadonlyMap<StorageId, StorageStatsTable>;
  readonly wall: WallStatsTable;
}
