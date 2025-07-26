// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { AcademyRecruitOrderImpl } from './recruit-order';
import { InfrastructureQueueImpl } from '@/core/model/infrastructure/queue/queue';

export class AcademyRecruitQueueImpl extends InfrastructureQueueImpl<AcademyRecruitOrderImpl>
  implements AcademyRecruitQueue
{
  private constructor(queue: AcademyRecruitQueue) {
    const orders = queue.orders.map((it) => AcademyRecruitOrderImpl.create(it));
    super({ orders });
  }

  public static create(queue: AcademyRecruitQueue) {
    return new AcademyRecruitQueueImpl(queue);
  }
}
