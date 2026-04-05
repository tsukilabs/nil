// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit/math';
import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';
import type { ContinentKey } from '@/types/core/continent';
import type { CheatSetStabilityRequest } from '@/types/request/cheat/city';

export async function cheatSetStability(coord: ContinentKey, stability: number) {
  coord = CoordImpl.fromContinentKey(coord);
  stability = clamp(stability, 0.0, 1.0);

  const req: CheatSetStabilityRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    stability,
  };

  await invoke('cheat_set_stability', { req });
}
