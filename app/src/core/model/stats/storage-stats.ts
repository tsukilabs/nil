// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class StorageStatsImpl implements StorageStats {
  public readonly level: BuildingLevel;
  public readonly capacity: number;

  private constructor(stats: StorageStats) {
    this.level = stats.level;
    this.capacity = stats.capacity;
  }

  public static create(stats: StorageStats) {
    return new StorageStatsImpl(stats);
  }
}
