// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit';
import { toU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';
import { SquadImpl } from '@/core/model/military/squad';

export async function simulateBattle(args: {
  attacker?: Option<readonly Squad[]>;
  defender?: Option<readonly Squad[]>;
  wall?: Option<BuildingLevel>;
}) {
  args.attacker ??= [];
  args.defender ??= [];
  args.wall ??= 0;

  if (args.attacker.length > 0) {
    args.attacker = args.attacker.map(SquadImpl.normalize.bind(SquadImpl));
  }

  if (args.defender.length > 0) {
    args.defender = args.defender.map(SquadImpl.normalize.bind(SquadImpl));
  }

  const stats = NIL.world.getStats();
  if (stats) {
    const minWall = stats.getBuildingMinLevel('wall');
    const maxWall = stats.getBuildingMaxLevel('wall');
    if (typeof minWall === 'number' && typeof maxWall === 'number') {
      args.wall = clamp(toU8(args.wall), minWall, maxWall);
    }
  }

  return invoke<BattleResult>('simulate_battle', { req: args });
}
