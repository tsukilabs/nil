// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { BuildingId, BuildingLevel } from '@/types/core/infrastructure/building';

export interface CheatGetAcademyRecruitQueueRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetAcademyRecruitQueuesRequest {
  readonly world: WorldId;
  readonly coords: readonly Coord[];
  readonly filterEmpty: boolean;
}

export interface CheatGetAllAcademyRecruitQueuesRequest {
  readonly world: WorldId;
  readonly filterEmpty: boolean;
}

export interface CheatGetAllPrefectureBuildQueuesRequest {
  readonly world: WorldId;
  readonly filterEmpty: boolean;
}

export interface CheatGetAllStableRecruitQueuesRequest {
  readonly world: WorldId;
  readonly filterEmpty: boolean;
}

export interface CheatGetInfrastructureRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetPrefectureBuildQueueRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetPrefectureBuildQueuesRequest {
  readonly world: WorldId;
  readonly coords: readonly Coord[];
  readonly filterEmpty: boolean;
}

export interface CheatGetStableRecruitQueueRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface CheatGetStableRecruitQueuesRequest {
  readonly world: WorldId;
  readonly coords: readonly Coord[];
  readonly filterEmpty: boolean;
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
