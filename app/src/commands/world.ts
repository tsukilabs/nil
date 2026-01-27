// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { SavedataInfo } from '@/core/savedata';
import { WorldConfigImpl } from '@/core/model/world-config';
import { type RawWorldStats, WorldStatsImpl } from '@/core/model/stats/world-stats';
import type {
  GetWorldConfigRequest,
  GetWorldStatsRequest,
  SaveWorldRequest as SaveLocalWorldRequest,
} from '@/lib/request';

export async function getWorldConfig(world?: Option<WorldId>): Promise<WorldConfigImpl> {
  const req: GetWorldConfigRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  const config = await invoke<WorldConfig>('get_world_config', { req });
  return WorldConfigImpl.create(config);
}

export async function getWorldStats(world?: Option<WorldId>): Promise<WorldStatsImpl> {
  const req: GetWorldStatsRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  const stats = await invoke<RawWorldStats>('get_world_stats', { req });
  return WorldStatsImpl.fromRaw(stats);
}

export async function readSavedataInfo(path: string) {
  return invoke<SavedataInfo>('read_savedata_info', { path });
}

export async function saveLocalWorld(path: string) {
  const req: SaveLocalWorldRequest = {
    world: NIL.world.getIdStrict(),
    path,
  };

  await invoke('save_local_world', { req });
}
