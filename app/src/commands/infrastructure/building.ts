// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type { BuildingId, Coord, ToggleBuildingRequest } from "@tsukilabs/nil-bindings";

export async function toggleBuilding(coord: Coord, id: BuildingId, enabled: boolean) {
  const req: ToggleBuildingRequest = {
    world: NIL.world.getIdStrict(),
    id,
    coord,
    enabled,
  };

  await invoke("toggle_building", { req });
}
