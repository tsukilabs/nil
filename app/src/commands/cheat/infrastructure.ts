// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { getCityOwner } from '@/commands/city';
import { CoordImpl } from '@/core/model/continent/coord';
import type {
  CheatGetAcademyRecruitQueueRequest,
  CheatGetInfrastructureRequest,
  CheatGetPrefectureBuildQueueRequest,
  CheatGetStableRecruitQueueRequest,
  CheatGetStorageCapacityRequest,
  CheatSetBuildingLevelRequest,
  CheatSetMaxInfrastructureRequest,
} from '@/lib/request';

export async function cheatGetAcademyRecruitQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetAcademyRecruitQueueRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke('cheat_get_academy_recruit_queue', { req });
}

export async function cheatGetInfrastructure(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetInfrastructureRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke('cheat_get_infrastructure', { req });
}

export async function cheatGetPrefectureBuildQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetPrefectureBuildQueueRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke('cheat_get_prefecture_build_queue', { req });
}

export async function cheatGetStableRecruitQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetStableRecruitQueueRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke('cheat_get_stable_recruit_queue', { req });
}

export async function cheatGetStorageCapacity(ruler?: Option<Ruler>) {
  const req: CheatGetStorageCapacityRequest = {
    world: NIL.world.getIdStrict(),
    ruler: ruler ?? null,
  };

  return invoke<OverallStorageCapacity>('cheat_get_storage_capacity', { req });
}

export async function cheatGetOwnerStorageCapacity(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatGetStorageCapacity(ruler);
}

export async function cheatSetBuildingLevel(
  coord: ContinentKey,
  id: BuildingId,
  level: BuildingLevel,
) {
  coord = CoordImpl.fromContinentKey(coord);
  level = toU8(level);

  const req: CheatSetBuildingLevelRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    id,
    level,
  };

  await invoke('cheat_set_building_level', { req });
}

export async function cheatSetMaxInfrastructure(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatSetMaxInfrastructureRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  await invoke('cheat_set_max_infrastructure', { req });
}
