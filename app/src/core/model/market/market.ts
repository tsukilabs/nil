// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import type { DeepReadonly } from "es-toolkit/types";
import type { Market, MarketFee } from "@tsukilabs/nil-bindings";
import { MarketVaultImpl } from "@/core/model/market/market-vault";

export class MarketImpl implements DeepReadonly<Market> {
  public readonly vault: MarketVaultImpl;
  public readonly fee: MarketFee;

  private constructor(market: Market) {
    this.vault = MarketVaultImpl.create(market.vault);
    this.fee = market.fee;
  }

  public static create(market: Market) {
    if (market instanceof MarketImpl) {
      return market;
    }

    return new MarketImpl(market);
  }

  public static async load() {
    const market = await commands.getMarket();
    return MarketImpl.create(market);
  }
}
