// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type {
  CheatSetMarketFeeRequest,
  CheatSetMarketVaultResourcesRequest,
  MarketFee,
  Resources,
} from "@tsukilabs/nil-bindings";

export async function cheatSetMarketFee(fee: MarketFee) {
  const req: CheatSetMarketFeeRequest = {
    world: NIL.world.getIdStrict(),
    fee,
  };

  await invoke("cheat_set_market_fee", { req });
}

export async function cheatSetMarketVaultResources(resources: Resources) {
  const req: CheatSetMarketVaultResourcesRequest = {
    world: NIL.world.getIdStrict(),
    resources,
  };

  await invoke("cheat_set_market_vault_resources", { req });
}
