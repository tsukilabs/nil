// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { BuildStep } from '@/types/core/behavior';
import { CoordImpl } from '@/core/model/continent/coord';
import type { ContinentKey } from '@/types/core/continent';
import type { CheatGetBuildStepsRequest } from '@/types/request/cheat/behavior';

export async function cheatGetBuildSteps(
  coord: ContinentKey,
  limit = Number.MAX_SAFE_INTEGER,
) {
  coord = CoordImpl.fromContinentKey(coord);
  const req: CheatGetBuildStepsRequest = {
    world: NIL.world.getIdStrict(),
    coord,
  };

  const steps = await invoke<BuildStep[]>('cheat_get_build_steps', { req });
  while (steps.length > limit) steps.pop();
  return steps as readonly BuildStep[];
}
