// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { MarketScene } from "@/types/scene";
import type { RouteRecordRaw } from "vue-router";

export const marketRoutes: RouteRecordRaw[] = [
  {
    component: () => import("@/scenes/game/market/root/index.vue"),
    path: "",
    name: "market" satisfies MarketScene,
  },
  {
    component: () => import("@/scenes/game/market/send/index.vue"),
    path: "send",
    name: "market-send" satisfies MarketScene,
  },
];
