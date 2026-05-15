// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { type Option, unwrap } from "@tb-dev/utils";
import { MineStatsTableImpl } from "./mine-stats-table";
import { StorageStatsTableImpl } from "./storage-stats-table";
import { BuildingStatsTableImpl } from "./building-stats-table";
import { WallStatsTableImpl } from "@/core/model/stats/wall-stats-table";
import type {
  BuildingId,
  BuildingStatsTable,
  InfrastructureStats,
  MineId,
  MineStatsTable,
  StorageId,
  StorageStatsTable,
} from "@tsukilabs/nil-bindings";

export class InfrastructureStatsImpl {
  public readonly building: ReadonlyMap<BuildingId, BuildingStatsTableImpl>;
  public readonly mine: ReadonlyMap<MineId, MineStatsTableImpl>;
  public readonly storage: ReadonlyMap<StorageId, StorageStatsTableImpl>;
  public readonly wall: WallStatsTableImpl;

  private constructor(args: InfrastructureStatsImplConstructorArgs) {
    this.building = args.building;
    this.mine = args.mine;
    this.storage = args.storage;
    this.wall = args.wall;
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

  public static fromRaw(raw: InfrastructureStats) {
    interface UninitInfrastructureStats {
      building: Map<BuildingId, BuildingStatsTableImpl>;
      mine: Map<MineId, MineStatsTableImpl>;
      storage: Map<StorageId, StorageStatsTableImpl>;
      wall: Option<WallStatsTableImpl>;
    }

    const infrastructure: UninitInfrastructureStats = {
      building: new Map<BuildingId, BuildingStatsTableImpl>(),
      mine: new Map<MineId, MineStatsTableImpl>(),
      storage: new Map<StorageId, StorageStatsTableImpl>(),
      wall: null,
    };

    for (const [key, value] of Object.entries(raw)) {
      switch (key as keyof InfrastructureStats) {
        case "building": {
          type Entries = [BuildingId, BuildingStatsTable][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const impl = BuildingStatsTableImpl.fromRaw(record);
            infrastructure.building.set(id, impl);
          }

          break;
        }
        case "mine": {
          type Entries = [MineId, MineStatsTable][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const impl = MineStatsTableImpl.fromRaw(record);
            infrastructure.mine.set(id, impl);
          }

          break;
        }
        case "storage": {
          type Entries = [StorageId, StorageStatsTable][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const impl = StorageStatsTableImpl.fromRaw(record);
            infrastructure.storage.set(id, impl);
          }

          break;
        }
        case "wall": {
          infrastructure.wall = WallStatsTableImpl.fromRaw(value);
          break;
        }
      }
    }

    return new InfrastructureStatsImpl({
      building: infrastructure.building,
      mine: infrastructure.mine,
      storage: infrastructure.storage,
      wall: unwrap(infrastructure.wall),
    });
  }
}

interface InfrastructureStatsImplConstructorArgs {
  readonly building: ReadonlyMap<BuildingId, BuildingStatsTableImpl>;
  readonly mine: ReadonlyMap<MineId, MineStatsTableImpl>;
  readonly storage: ReadonlyMap<StorageId, StorageStatsTableImpl>;
  readonly wall: WallStatsTableImpl;
}
