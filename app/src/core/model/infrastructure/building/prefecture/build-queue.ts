// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { PrefectureBuildOrderImpl } from './build-order';
import { InfrastructureQueueImpl } from '@/core/model/infrastructure/queue/queue';

export class PrefectureBuildQueueImpl extends InfrastructureQueueImpl<PrefectureBuildOrderImpl>
  implements PrefectureBuildQueue
{
  private constructor(queue: PrefectureBuildQueue) {
    const orders = queue.orders.map((it) => PrefectureBuildOrderImpl.create(it));
    super({ orders });
  }

  public static create(queue: PrefectureBuildQueue) {
    return new PrefectureBuildQueueImpl(queue);
  }
}
