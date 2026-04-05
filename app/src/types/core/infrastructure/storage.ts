// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingLevel } from '@/types/core/infrastructure/building';

export type StorageId = 'silo' | 'warehouse';

export interface StorageStats {
  readonly capacity: number;
  readonly level: BuildingLevel;
}

export interface StorageStatsTable {
  readonly id: StorageId;
  readonly table: ReadonlyMap<BuildingLevel, StorageStats>;
}

export interface OverallStorageCapacity {
  readonly silo: number;
  readonly warehouse: number;
}
