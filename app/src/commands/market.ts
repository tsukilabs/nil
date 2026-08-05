// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type {
  BuyResourcesRequest,
  GetMarketFeeRequest,
  GetMarketFeeResponse,
  GetMarketRequest,
  GetMarketResponse,
  Resources,
  Ruler,
  SellResourcesRequest,
  SendResourcesRequest,
} from "@tsukilabs/nil-bindings";

export async function buyResources(resources: Resources) {
  const req: BuyResourcesRequest = {
    world: NIL.world.getIdStrict(),
    resources,
  };

  await invoke("buy_resources", { req });
}

export async function getMarket() {
  const req: GetMarketRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetMarketResponse>("get_market", { req });
}

export async function getMarketFee() {
  const req: GetMarketFeeRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetMarketFeeResponse>("get_market_fee", { req });
}

export async function sellResources(resources: Resources) {
  const req: SellResourcesRequest = {
    world: NIL.world.getIdStrict(),
    resources,
  };

  await invoke("sell_resources", { req });
}

export async function sendResources(recipient: Ruler, resources: Resources) {
  const req: SendResourcesRequest = {
    world: NIL.world.getIdStrict(),
    recipient,
    resources,
  };

  await invoke("send_resources", { req });
}
