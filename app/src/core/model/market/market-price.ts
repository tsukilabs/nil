// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { DeepReadonly } from "es-toolkit/types";
import type { Gold, MarketPrice } from "@tsukilabs/nil-bindings";

export class MarketPriceImpl implements DeepReadonly<MarketPrice> {
  public readonly food: Gold;
  public readonly iron: Gold;
  public readonly stone: Gold;
  public readonly wood: Gold;

  private constructor(price: MarketPrice) {
    this.food = price.food;
    this.iron = price.iron;
    this.stone = price.stone;
    this.wood = price.wood;
  }

  public static create(price: MarketPrice) {
    if (price instanceof MarketPriceImpl) {
      return price;
    }

    return new MarketPriceImpl(price);
  }
}
