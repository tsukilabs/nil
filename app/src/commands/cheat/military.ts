// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

export async function cheatSpawnPersonnel(
  coord: ContinentKey,
  ruler?: Option<Ruler>,
  personnel?: Option<ArmyPersonnel | number>,
) {
  coord = CoordImpl.fromKey(coord);
  ruler ??= null;
  personnel ??= 1_000;

  if (typeof personnel === 'number') {
    personnel = ArmyPersonnelImpl.splat(personnel);
  }

  await invoke('cheat_spawn_personnel', { req: { coord, ruler, personnel } });
}
