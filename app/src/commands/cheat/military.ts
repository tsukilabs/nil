// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { SquadImpl } from '@/core/model/military/squad';
import { CoordImpl } from '@/core/model/continent/coord';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';
import type {
  CheatGetIdleArmiesAtRequest,
  CheatGetIdlePersonnelAtRequest,
  CheatSpawnPersonnelRequest,
} from '@/lib/request';

export async function cheatGetIdleArmiesAt(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetIdleArmiesAtRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<readonly Army[]>('cheat_get_idle_armies_at', { req });
}

export async function cheatGetIdlePersonnelAt(coord?: Option<ContinentKey>) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  const req: CheatGetIdlePersonnelAtRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  return invoke<ArmyPersonnel>('cheat_get_idle_personnel_at', { req });
}

export async function cheatGetIdlePersonnelSizeAt(coord?: Option<ContinentKey>) {
  const personnel = await cheatGetIdlePersonnelAt(coord);
  return ArmyPersonnelImpl.create(personnel).getSize();
}

export async function cheatGetIdleSquadsAt(coord?: Option<ContinentKey>) {
  const armies = await cheatGetIdleArmiesAt(coord);
  return armies.flatMap((army) => {
    const personnel = ArmyPersonnelImpl.create(army.personnel);
    return personnel.getSquads();
  });
}

export async function cheatGetIdleUnitAmountAt(coord?: Option<ContinentKey>) {
  const size = await cheatGetIdlePersonnelSizeAt(coord);
  return Object.values(size).reduce((acc, curr) => acc + curr, 0);
}

export async function cheatSpawnPersonnel(
  coord?: Option<ContinentKey>,
  personnel?: Option<ArmyPersonnel | number>,
  ruler?: Option<Ruler>,
) {
  coord = CoordImpl.fromContinentKeyOrCurrentStrict(coord);
  personnel ??= 1_000;
  ruler ??= null;

  if (typeof personnel === 'number') {
    personnel = ArmyPersonnelImpl.splat(personnel);
  }

  const req: CheatSpawnPersonnelRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    personnel,
    ruler,
  };

  await invoke('cheat_spawn_personnel', { req });
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
