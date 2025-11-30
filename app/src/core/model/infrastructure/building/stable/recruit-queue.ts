// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { StableRecruitOrderImpl } from './recruit-order';
import { InfrastructureQueueImpl } from '@/core/model/infrastructure/queue/queue';

export class StableRecruitQueueImpl extends InfrastructureQueueImpl<StableRecruitOrderImpl>
  implements StableRecruitQueue
{
  private constructor(queue: StableRecruitQueue) {
    const orders = queue.orders.map((it) => StableRecruitOrderImpl.create(it));
    super({ orders });
  }

  public static create(queue: StableRecruitQueue) {
    return new StableRecruitQueueImpl(queue);
  }
}
