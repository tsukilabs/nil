// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { ToggleBuildingRequest } from '@/lib/request';

export * from './stable';
export * from './academy';
export * from './workshop';
export * from './prefecture';

export async function toggleBuilding(coord: Coord, id: BuildingId, enabled: boolean) {
  const req: ToggleBuildingRequest = {
    world: NIL.world.getIdStrict(),
    id,
    coord,
    enabled,
  };

  await invoke('toggle_building', { req });
}
