// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { SquadImpl } from '@/core/model/military/squad';
import { InfrastructureQueueOrderImpl } from '@/core/model/infrastructure/queue/queue-order';

export class StableRecruitOrderImpl extends InfrastructureQueueOrderImpl
  implements StableRecruitOrder
{
  public readonly squad: SquadImpl;

  private constructor(order: StableRecruitOrder) {
    super(order);

    this.squad = SquadImpl.create(order.squad);
  }

  public static create(order: StableRecruitOrder) {
    return new StableRecruitOrderImpl(order);
  }
}
