// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { WorkshopRecruitOrderImpl } from './recruit-order';
import { InfrastructureQueueImpl } from '@/core/model/infrastructure/queue/queue';

export class WorkshopRecruitQueueImpl extends InfrastructureQueueImpl<WorkshopRecruitOrderImpl>
  implements WorkshopRecruitQueue
{
  private constructor(queue: WorkshopRecruitQueue) {
    const orders = queue.orders.map((it) => WorkshopRecruitOrderImpl.create(it));
    super({ orders });
  }

  public static create(queue: WorkshopRecruitQueue) {
    return new WorkshopRecruitQueueImpl(queue);
  }
}
