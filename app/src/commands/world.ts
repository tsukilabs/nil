// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { WorldConfigImpl } from '@/core/model/world-config';
import { type RawWorldStats, WorldStatsImpl } from '@/core/model/stats/world-stats';

export async function getWorldConfig(): Promise<WorldConfigImpl> {
  const config = await invoke<WorldConfig>('get_world_config');
  return WorldConfigImpl.create(config);
}

export async function getWorldStats(): Promise<WorldStatsImpl> {
  const stats = await invoke<RawWorldStats>('get_world_stats');
  return WorldStatsImpl.fromRaw(stats);
}

export async function saveWorld(path: string) {
  await invoke('save_world', { req: { path } });
}
