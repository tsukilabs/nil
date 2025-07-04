// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { MineStatsTableImpl, type RawMineStatsTable } from './mine-stats-table';
import { type RawStorageStatsTable, StorageStatsTableImpl } from './storage-stats-table';
import { BuildingStatsTableImpl, type RawBuildingStatsTable } from './building-stats-table';

export class InfrastructureStatsImpl implements InfrastructureStats {
  public readonly building: ReadonlyMap<BuildingId, BuildingStatsTableImpl>;
  public readonly mine: ReadonlyMap<MineId, MineStatsTableImpl>;
  public readonly storage: ReadonlyMap<StorageId, StorageStatsTableImpl>;

  private constructor(args: {
    building: ReadonlyMap<BuildingId, BuildingStatsTableImpl>;
    mine: ReadonlyMap<MineId, MineStatsTableImpl>;
    storage: ReadonlyMap<StorageId, StorageStatsTableImpl>;
  }) {
    this.building = args.building;
    this.mine = args.mine;
    this.storage = args.storage;
  }

  public getBuilding(building: BuildingId) {
    return this.building.get(building);
  }

  public getMine(mine: MineId) {
    return this.mine.get(mine);
  }

  public getStorage(storage: StorageId) {
    return this.storage.get(storage);
  }

  public getMinLevel(building: BuildingId) {
    return this.getBuilding(building)?.minLevel;
  }

  public getMaxLevel(building: BuildingId) {
    return this.getBuilding(building)?.maxLevel;
  }

  public static fromRaw(raw: RawInfrastructureStats) {
    const infrastructure = {
      building: new Map<BuildingId, BuildingStatsTableImpl>(),
      mine: new Map<MineId, MineStatsTableImpl>(),
      storage: new Map<StorageId, StorageStatsTableImpl>(),
    };

    for (const [key, value] of Object.entries(raw)) {
      switch (key as keyof RawInfrastructureStats) {
        case 'building': {
          type Entries = [BuildingId, RawBuildingStatsTable][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const impl = BuildingStatsTableImpl.fromRaw(record);
            infrastructure.building.set(id, impl);
          }

          break;
        }
        case 'mine': {
          type Entries = [MineId, RawMineStatsTable][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const impl = MineStatsTableImpl.fromRaw(record);
            infrastructure.mine.set(id, impl);
          }

          break;
        }
        case 'storage': {
          type Entries = [StorageId, RawStorageStatsTable][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const impl = StorageStatsTableImpl.fromRaw(record);
            infrastructure.storage.set(id, impl);
          }

          break;
        }
      }
    }

    return new InfrastructureStatsImpl(infrastructure);
  }
}

export interface RawInfrastructureStats {
  readonly building: Record<BuildingId, RawBuildingStatsTable>;
  readonly mine: Record<MineId, RawMineStatsTable>;
  readonly storage: Record<StorageId, RawStorageStatsTable>;
}
