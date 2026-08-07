// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import type { PartialNullish } from "@tb-dev/utils";
import type { DeepReadonly } from "es-toolkit/types";
import { MarketVaultImpl } from "@/core/model/market/market-vault";
import { MarketPriceTableImpl } from "@/core/model/market/market-price";
import type { Market, MarketFee, Resources, Ruler } from "@tsukilabs/nil-bindings";

export class MarketImpl implements DeepReadonly<Market> {
  public readonly vault: MarketVaultImpl;
  public readonly fee: MarketFee;
  public readonly priceTable: MarketPriceTableImpl;

  private constructor(market: Market) {
    this.vault = MarketVaultImpl.create(market.vault);
    this.fee = market.fee;
    this.priceTable = MarketPriceTableImpl.create(market.priceTable);
  }

  public async buy(resources: Resources) {
    return commands.buyResources(resources);
  }

  public async sell(resources: Resources) {
    return commands.sellResources(resources);
  }

  public async send(recipient: Ruler, resources: Resources) {
    return commands.sendResources(recipient, resources);
  }

  public hasResourcesInVault(resources: PartialNullish<Resources>) {
    return this.vault.hasResources(resources);
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
