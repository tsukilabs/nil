// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type StorageId = 'silo' | 'warehouse';

interface StorageStats {
  readonly capacity: number;
  readonly level: BuildingLevel;
}

interface StorageStatsTable {
  readonly id: StorageId;
  readonly table: ReadonlyMap<BuildingLevel, StorageStats>;
}

interface OverallStorageCapacity {
  readonly silo: number;
  readonly warehouse: number;
}
