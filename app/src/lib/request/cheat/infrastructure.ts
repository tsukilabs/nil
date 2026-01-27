// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface CheatGetAcademyRecruitQueueRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetInfrastructureRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetPrefectureBuildQueueRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetStableRecruitQueueRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetStorageCapacityRequest {
  readonly world: WorldId;
  readonly ruler: Option<Ruler>;
}

export interface CheatSetBuildingLevelRequest {
  readonly world: WorldId;
  readonly coord: Coord;
  readonly id: BuildingId;
  readonly level: BuildingLevel;
}

export interface CheatSetMaxInfrastructureRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}
