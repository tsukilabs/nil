// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { SquadImpl } from '@/core/model/military/squad';
import { InfrastructureQueueOrderImpl } from '@/core/model/infrastructure/queue/queue-order';

export class AcademyRecruitOrderImpl extends InfrastructureQueueOrderImpl
  implements AcademyRecruitOrder
{
  public readonly squad: SquadImpl;

  private constructor(order: AcademyRecruitOrder) {
    super(order);

    this.squad = SquadImpl.create(order.squad);
  }

  public static create(order: AcademyRecruitOrder) {
    return new AcademyRecruitOrderImpl(order);
  }
}
