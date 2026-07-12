// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from "es-toolkit/math";
import type { Option } from "@tb-dev/utils";
import { invoke } from "@tauri-apps/api/core";
import { CoordImpl } from "@/core/model/continent/coord";
import type { ContinentKey } from "@/types/core/continent";
import type {
  BotId,
  CheatGetCitiesRequest,
  CheatGetCityRequest,
  CheatGetCityResponse,
  CheatSetStabilityRequest,
  CheatSpawnCityRequest,
  PlayerId,
  PrecursorId,
  Ruler,
} from "@tsukilabs/nil-bindings";

export async function cheatGetCities(options: {
  coords?: Option<ContinentKey[]>;
  score?: Option<boolean>;
  all?: Option<boolean>;
}) {
  const coords = options.coords?.map((coord) => CoordImpl.fromContinentKey(coord));
  const req: CheatGetCitiesRequest = {
    world: NIL.world.getIdStrict(),
    coords: coords ?? [],
    score: options.score ?? false,
    all: options.all ?? false,
  };

  return invoke<readonly CheatGetCityResponse[]>("cheat_get_cities", { req });
}

export async function cheatGetCity(options: {
  coord: ContinentKey;
  score?: Option<boolean>;
}) {
  const req: CheatGetCityRequest = {
    world: NIL.world.getIdStrict(),
    coord: CoordImpl.fromContinentKey(options.coord),
    score: options.score ?? false,
  };

  return invoke<CheatGetCityResponse>("cheat_get_city", { req });
}

export async function cheatSetStability(coord: ContinentKey, stability: number) {
  coord = CoordImpl.fromContinentKey(coord);
  stability = clamp(stability, 0.0, 1.0);

  const req: CheatSetStabilityRequest = {
    world: NIL.world.getIdStrict(),
    coord,
    stability,
  };

  await invoke("cheat_set_stability", { req });
}

export async function cheatSpawnCity(ruler: Ruler, coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  const req: CheatSpawnCityRequest = {
    world: NIL.world.getIdStrict(),
    ruler,
    coord,
  };

  await invoke("cheat_spawn_city", { req });
}

export async function cheatSpawnBotCity(id: BotId, coord: ContinentKey) {
  return cheatSpawnCity({ kind: "bot", id }, coord);
}

export async function cheatSpawnPlayerCity(id: PlayerId, coord: ContinentKey) {
  return cheatSpawnCity({ kind: "player", id }, coord);
}

export async function cheatSpawnPrecursorCity(id: PrecursorId, coord: ContinentKey) {
  return cheatSpawnCity({ kind: "precursor", id }, coord);
}
