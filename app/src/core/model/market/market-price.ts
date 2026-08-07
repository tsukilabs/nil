// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { DeepReadonly } from "es-toolkit/types";
import type { Gold, MarketPriceTable } from "@tsukilabs/nil-bindings";

export class MarketPriceTableImpl implements DeepReadonly<MarketPriceTable> {
  public readonly food: Gold;
  public readonly iron: Gold;
  public readonly stone: Gold;
  public readonly wood: Gold;

  private constructor(price: MarketPriceTable) {
    this.food = price.food;
    this.iron = price.iron;
    this.stone = price.stone;
    this.wood = price.wood;
  }

  public static create(price: MarketPriceTable) {
    if (price instanceof MarketPriceTableImpl) {
      return price;
    }

    return new MarketPriceTableImpl(price);
  }
}
