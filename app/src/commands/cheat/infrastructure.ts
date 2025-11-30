// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';

export function cheatSetBuildingLevel(coord: Coord, id: BuildingId, level: BuildingLevel) {
  level = toU8(level);
  return invoke<null>('cheat_set_building_level', { coord, id, level });
}

export function cheatSetMaxInfrastructure(coord?: Option<Coord>) {
  coord ??= NIL.city.refs().coord.value;
  return invoke<null>('cheat_set_max_infrastructure', { coord });
}
