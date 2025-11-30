// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export * from './stable';
export * from './academy';
export * from './prefecture';

export async function toggleBuilding(coord: Coord, id: BuildingId, enabled: boolean) {
  await invoke('toggle_building', { req: { coord, id, enabled } });
}
