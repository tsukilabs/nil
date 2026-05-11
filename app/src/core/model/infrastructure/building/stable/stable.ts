// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from "../abstract";
import { StableRecruitQueueImpl } from "./recruit-queue";
import type { BuildingId, Stable, StableRecruitOrderId } from "@tsukilabs/nil-bindings";

export class StableImpl extends BuildingImpl implements Readonly<Stable> {
  public readonly id: BuildingId = "stable";
  public readonly recruitQueue: StableRecruitQueueImpl;

  private constructor(stable: Stable) {
    super(stable);

    this.recruitQueue = StableRecruitQueueImpl.create(stable.recruitQueue);
  }

  public hasRecruitOrder(id: StableRecruitOrderId) {
    return this.recruitQueue.orders.some((order) => order.id === id);
  }

  public static create(stable: Stable) {
    return new StableImpl(stable);
  }
}
