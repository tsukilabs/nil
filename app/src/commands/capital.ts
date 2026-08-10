// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type { GetCityLimitRequest, GetCityLimitResponse } from "@tsukilabs/nil-bindings";

export async function getCityLimit() {
  const req: GetCityLimitRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetCityLimitResponse>("get_city_limit", { req });
}
