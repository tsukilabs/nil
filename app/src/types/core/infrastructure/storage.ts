// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingLevel, StorageId, StorageStats } from '@/types/bindings';

export interface StorageStatsTable {
  readonly id: StorageId;
  readonly table: ReadonlyMap<BuildingLevel, StorageStats>;
}
