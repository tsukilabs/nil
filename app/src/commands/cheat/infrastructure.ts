// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';

export async function cheatGetAcademyRecruitQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrent(coord);
  return invoke('cheat_get_academy_recruit_queue', { req: { coord } });
}

export async function cheatGetInfrastructure(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrent(coord);
  return invoke('cheat_get_infrastructure', { req: { coord } });
}

export async function cheatGetPrefectureBuildQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrent(coord);
  return invoke('cheat_get_prefecture_build_queue', { req: { coord } });
}

export async function cheatGetStableRecruitQueue(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrent(coord);
  return invoke('cheat_get_stable_recruit_queue', { req: { coord } });
}

export async function cheatGetStorageCapacity(ruler?: Option<Ruler>) {
  return invoke<OverallStorageCapacity>('cheat_get_storage_capacity', { req: { ruler } });
}

export async function cheatSetBuildingLevel(
  coord: ContinentKey,
  id: BuildingId,
  level: BuildingLevel,
) {
  coord = CoordImpl.fromContinentKey(coord);
  level = toU8(level);
  await invoke('cheat_set_building_level', { req: { coord, id, level } });
}

export async function cheatSetMaxInfrastructure(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrent(coord);
  await invoke('cheat_set_max_infrastructure', { req: { coord } });
}
