// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from "vue";
import { throttle } from "es-toolkit/function";
import { asyncRef, useMutex } from "@tb-dev/vue";
import { MarketImpl } from "@/core/model/market/market";
import type { Resources, Ruler } from "@tsukilabs/nil-bindings";

export function useMarket() {
  const market = asyncRef(null, () => MarketImpl.load());
  const throttledLoad = throttle(market.load, 1000, {
    edges: ["leading", "trailing"],
  });

  const { locked, lock } = useMutex();

  const loading = computed(() => {
    return market.loading.value || locked.value;
  });

  async function buyResources(resources: Resources) {
    await lock(async () => {
      await market.state.value?.buy(resources);
    });
  }

  async function sellResources(resources: Resources) {
    await lock(async () => {
      await market.state.value?.sell(resources);
    });
  }

  async function sendResources(recipient: Ruler, resources: Resources) {
    await lock(async () => {
      await market.state.value?.send(recipient, resources);
    });
  }

  return {
    market: market.state,
    loading,
    load: market.load,
    throttledLoad,
    buyResources,
    sellResources,
    sendResources,
  };
}
