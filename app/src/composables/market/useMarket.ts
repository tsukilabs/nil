// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { asyncRef } from "@tb-dev/vue";
import { MarketImpl } from "@/core/model/market/market";

export function useMarket() {
  const market = asyncRef(null, () => MarketImpl.load());

  return {
    market: market.state,
    loading: market.loading,
    load: market.load,
  };
}
