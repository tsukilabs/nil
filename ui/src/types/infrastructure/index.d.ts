// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Infrastructure {
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
}

interface InfrastructureStats {
  readonly building: ReadonlyMap<BuildingId, BuildingStatsTable>;
  readonly mine: ReadonlyMap<MineId, MineStatsTable>;
  readonly storage: ReadonlyMap<StorageId, StorageStatsTable>;
  readonly wall: WallStatsTable;
}

type InfrastructureRequirements = {
  readonly [B in keyof Infrastructure]: BuildingLevel;
};
