// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { SquadImpl } from '@/core/model/military/squad';
import { InfrastructureQueueOrderImpl } from '@/core/model/infrastructure/queue/queue-order';

export class WorkshopRecruitOrderImpl extends InfrastructureQueueOrderImpl
  implements WorkshopRecruitOrder
{
  public readonly squad: SquadImpl;

  private constructor(order: WorkshopRecruitOrder) {
    super(order);

    this.squad = SquadImpl.create(order.squad);
  }

  public static create(order: WorkshopRecruitOrder) {
    return new WorkshopRecruitOrderImpl(order);
  }
}
