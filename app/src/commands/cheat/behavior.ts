// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';

export async function cheatGetBuildSteps(
  coord: ContinentKey,
  limit = Number.MAX_SAFE_INTEGER,
) {
  coord = CoordImpl.fromContinentKey(coord);
  const steps = await invoke<BuildStep[]>('cheat_get_build_steps', { req: { coord } });
  while (steps.length > limit) steps.pop();
  return steps as readonly BuildStep[];
}
