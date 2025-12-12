// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { SquadImpl } from '@/core/model/military/squad';
import { CoordImpl } from '@/core/model/continent/coord';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';

export async function cheatSpawnPersonnel(
  coord: ContinentKey,
  personnel?: Option<ArmyPersonnel | number>,
  ruler?: Option<Ruler>,
) {
  coord = CoordImpl.fromContinentKey(coord);
  personnel ??= 1_000;
  ruler ??= null;

  if (typeof personnel === 'number') {
    personnel = ArmyPersonnelImpl.splat(personnel);
  }

  await invoke('cheat_spawn_personnel', { req: { coord, ruler, personnel } });
}

export async function cheatSpawnSquad(
  coord: ContinentKey,
  squad: Squad | SquadTuple | UnitId,
  ruler?: Option<Ruler>,
) {
  if (typeof squad === 'string') {
    squad = { unit: squad, size: 1_000 };
  }
  else if (Array.isArray(squad)) {
    squad = SquadImpl.fromTuple(squad);
  }

  const personnel = ArmyPersonnelImpl.fromSquad(squad);
  await cheatSpawnPersonnel(coord, personnel, ruler);
}
