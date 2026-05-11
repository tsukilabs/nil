// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import { isValidPassword } from "@/lib/schema";
import type { SavedataInfo } from "@/core/savedata";
import type { Option, Writable } from "@tb-dev/utils";
import { WorldConfigImpl } from "@/core/model/world-config";
import { WorldStatsImpl } from "@/core/model/stats/world-stats";
import type {
  CreateRemoteWorldRequest,
  CreateRemoteWorldResponse,
  DeleteRemoteWorldRequest,
  GetRemoteWorldLimitPerUserResponse,
  GetRemoteWorldLimitResponse,
  GetRemoteWorldRequest,
  GetRemoteWorldResponse,
  GetRemoteWorldsResponse,
  GetWorldBotsRequest,
  GetWorldBotsResponse,
  GetWorldConfigRequest,
  GetWorldConfigResponse,
  GetWorldPlayersRequest,
  GetWorldPlayersResponse,
  GetWorldPrecursorsRequest,
  GetWorldPrecursorsResponse,
  GetWorldStatsRequest,
  GetWorldStatsResponse,
  PlayerId,
  SaveLocalWorldRequest,
  WorldId,
} from "@tsukilabs/nil-bindings";

export async function createRemoteWorld(req: Writable<CreateRemoteWorldRequest>) {
  req.description &&= req.description.slice(0, 300);
  req.description ??= null;

  if (req.password && !isValidPassword(req.password)) {
    req.password = null;
  }

  return invoke<CreateRemoteWorldResponse>("create_remote_world", { req });
}

export async function deleteRemoteWorld(world?: Option<WorldId>) {
  const req: DeleteRemoteWorldRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  await invoke("delete_remote_world", { req });
}

export async function getRemoteWorld(world?: Option<WorldId>) {
  const req: GetRemoteWorldRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<GetRemoteWorldResponse>("get_remote_world", { req });
}

export async function getRemoteWorldLimit() {
  return invoke<GetRemoteWorldLimitResponse>("get_remote_world_limit");
}

export async function getRemoteWorldLimitPerUser() {
  return invoke<GetRemoteWorldLimitPerUserResponse>("get_remote_world_limit_per_user");
}

export async function getRemoteWorlds() {
  return invoke<GetRemoteWorldsResponse>("get_remote_worlds");
}

export async function getSavedataPlayers(path: string) {
  return invoke<readonly PlayerId[]>("get_savedata_players", { path });
}

export async function getWorldBots(world?: Option<WorldId>) {
  const req: GetWorldBotsRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<GetWorldBotsResponse>("get_world_bots", { req });
}

export async function getWorldConfig(world?: Option<WorldId>): Promise<WorldConfigImpl> {
  const req: GetWorldConfigRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  const config = await invoke<GetWorldConfigResponse>("get_world_config", { req });
  return WorldConfigImpl.create(config);
}

export async function getWorldPlayers(world?: Option<WorldId>) {
  const req: GetWorldPlayersRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<GetWorldPlayersResponse>("get_world_players", { req });
}

export async function getWorldPrecursors(world?: Option<WorldId>) {
  const req: GetWorldPrecursorsRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  return invoke<GetWorldPrecursorsResponse>("get_world_precursors", { req });
}

export async function getWorldStats(world?: Option<WorldId>): Promise<WorldStatsImpl> {
  const req: GetWorldStatsRequest = {
    world: world ?? NIL.world.getIdStrict(),
  };

  const stats = await invoke<GetWorldStatsResponse>("get_world_stats", { req });
  return WorldStatsImpl.fromRaw(stats);
}

export async function isSavedata(path: string) {
  return invoke<boolean>("is_savedata", { path });
}

export async function readSavedataInfo(path: string) {
  return invoke<SavedataInfo>("read_savedata_info", { path });
}

export async function saveLocalWorld(path: string) {
  const req: SaveLocalWorldRequest = {
    world: NIL.world.getIdStrict(),
    path,
  };

  await invoke("save_local_world", { req });
}

export async function savedataDir() {
  return invoke<string>("savedata_dir");
}
