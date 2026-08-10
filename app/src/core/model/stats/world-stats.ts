// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ResourcesImpl } from "@/core/model/resources";
import { MarketPriceTableImpl } from "@/core/model/market/market-price";
import { InfrastructureStatsImpl } from "@/core/model/stats/infrastructure-stats";
import type {
  BuildingId,
  InfluenceResourceCost,
  MarketPriceTable,
  WorldStats,
} from "@tsukilabs/nil-bindings";

export class WorldStatsImpl {
  public readonly infrastructure: InfrastructureStatsImpl;
  public readonly marketPriceTable: MarketPriceTableImpl;
  public readonly influenceResourceCost: ResourcesImpl;

  private constructor(args: WorldStatsImplConstructorArgs) {
    this.infrastructure = args.infrastructure;
    this.marketPriceTable = MarketPriceTableImpl.create(args.marketPriceTable);
    this.influenceResourceCost = ResourcesImpl.create(args.influenceResourceCost);
  }

  public getBuildingMinLevel(building: BuildingId) {
    return this.infrastructure.getMinLevel(building);
  }

  public getBuildingMaxLevel(building: BuildingId) {
    return this.infrastructure.getMaxLevel(building);
  }

  public static fromRaw(raw: WorldStats) {
    const infrastructure = InfrastructureStatsImpl.fromRaw(raw.infrastructure);
    return new WorldStatsImpl({
      infrastructure,
      marketPriceTable: raw.marketPriceTable,
      influenceResourceCost: raw.influenceResourceCost,
    });
  }
}

export interface WorldStatsImplConstructorArgs {
  infrastructure: InfrastructureStatsImpl;
  marketPriceTable: MarketPriceTable;
  influenceResourceCost: InfluenceResourceCost;
}
