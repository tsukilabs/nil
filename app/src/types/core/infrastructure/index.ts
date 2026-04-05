// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Stable } from '@/types/core/infrastructure/stable';
import type { Academy } from '@/types/core/infrastructure/academy';
import type { Workshop } from '@/types/core/infrastructure/workshop';
import type { Prefecture } from '@/types/core/infrastructure/prefecture';
import type { Wall, WallStatsTable } from '@/types/core/infrastructure/wall';
import type { MineId, MineStatsTable } from '@/types/core/infrastructure/mine';
import type { StorageId, StorageStatsTable } from '@/types/core/infrastructure/storage';
import type {
  BuildingId,
  BuildingLevel,
  BuildingStatsTable,
  Farm,
  IronMine,
  Quarry,
  Sawmill,
  Silo,
  Warehouse,
} from '@/types/core/infrastructure/building';

export interface Infrastructure {
  readonly academy: Academy;
  readonly farm: Farm;
  readonly ironMine: IronMine;
  readonly prefecture: Prefecture;
  readonly quarry: Quarry;
  readonly sawmill: Sawmill;
  readonly silo: Silo;
  readonly stable: Stable;
  readonly wall: Wall;
  readonly warehouse: Warehouse;
  readonly workshop: Workshop;
}

export interface InfrastructureStats {
  readonly building: ReadonlyMap<BuildingId, BuildingStatsTable>;
  readonly mine: ReadonlyMap<MineId, MineStatsTable>;
  readonly storage: ReadonlyMap<StorageId, StorageStatsTable>;
  readonly wall: WallStatsTable;
}

export type InfrastructureRequirements = {
  readonly [B in keyof Infrastructure]: BuildingLevel;
};
