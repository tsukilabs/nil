// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type {
  GetMarketFeeRequest,
  GetMarketFeeResponse,
  Resources,
  Ruler,
  SendResourcesRequest,
} from "@tsukilabs/nil-bindings";

export async function getMarketFee() {
  const req: GetMarketFeeRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetMarketFeeResponse>("get_market_fee", { req });
}

export async function sendResources(recipient: Ruler, resources: Resources) {
  const req: SendResourcesRequest = {
    world: NIL.world.getIdStrict(),
    recipient,
    resources,
  };

  await invoke("send_resources", { req });
}
