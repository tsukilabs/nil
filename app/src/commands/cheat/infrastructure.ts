// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';

export async function cheatGetStorageCapacity(ruler: Ruler) {
  return invoke<OverallStorageCapacity>('cheat_get_storage_capacity', { req: { ruler } });
}

export async function cheatSetBuildingLevel(coord: Coord, id: BuildingId, level: BuildingLevel) {
  level = toU8(level);
  await invoke('cheat_set_building_level', { req: { coord, id, level } });
}

export async function cheatSetMaxInfrastructure(coord?: Option<Coord>) {
  coord ??= NIL.city.getCoord();
  await invoke('cheat_set_max_infrastructure', { req: { coord } });
}
