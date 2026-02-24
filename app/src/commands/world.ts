// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { isValidPassword } from '@/lib/schema';
import type { SavedataInfo } from '@/core/savedata';
import { WorldConfigImpl } from '@/core/model/world-config';
import { type RawWorldStats, WorldStatsImpl } from '@/core/model/stats/world-stats';
import type {
  CreateRemoteWorldRequest,
  GetRemoteWorldRequest,
  GetWorldBotsRequest,
  GetWorldConfigRequest,
  GetWorldPlayersRequest,
  GetWorldPrecursorsRequest,
  GetWorldStatsRequest,
  SaveLocalWorldRequest,
} from '@/lib/request';

export async function createRemoteWorld(req: Writable<CreateRemoteWorldRequest>) {
  req.description &&= req.description.slice(0, 300);
  req.description ??= null;

  if (req.password && !isValidPassword(req.password)) {
    req.password = null;
  }

  return invoke<WorldId>('create_remote_world', { req });
}

export async function getRemoteWorld(world?: Option<WorldId>) {
  const req: GetRemoteWorldRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<RemoteWorld>('get_remote_world', { req });
}

export async function getRemoteWorlds() {
  return invoke<readonly RemoteWorld[]>('get_remote_worlds');
}

export async function getSavedataPlayers(path: string) {
  return invoke<readonly PlayerId[]>('get_savedata_players', { path });
}

export async function getWorldBots(world?: Option<WorldId>) {
  const req: GetWorldBotsRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<readonly BotId[]>('get_world_bots', { req });
}

export async function getWorldConfig(world?: Option<WorldId>): Promise<WorldConfigImpl> {
  const req: GetWorldConfigRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  const config = await invoke<WorldConfig>('get_world_config', { req });
  return WorldConfigImpl.create(config);
}

export async function getWorldPlayers(world?: Option<WorldId>) {
  const req: GetWorldPlayersRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<readonly PlayerId[]>('get_world_players', { req });
}

export async function getWorldPrecursors(world?: Option<WorldId>) {
  const req: GetWorldPrecursorsRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<readonly PrecursorId[]>('get_world_precursors', { req });
}

export async function getWorldStats(world?: Option<WorldId>): Promise<WorldStatsImpl> {
  const req: GetWorldStatsRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  const stats = await invoke<RawWorldStats>('get_world_stats', { req });
  return WorldStatsImpl.fromRaw(stats);
}

export async function isSavedata(path: string) {
  return invoke<boolean>('is_savedata', { path });
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

export async function savedataDir() {
  return invoke<string>('savedata_dir');
}
