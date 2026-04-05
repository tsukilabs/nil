// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from '../abstract';
import { WorkshopRecruitQueueImpl } from './recruit-queue';
import type { Workshop } from '@/types/core/infrastructure/workshop';
import type { BuildingId } from '@/types/core/infrastructure/building';
import type { InfrastructureQueueOrderId } from '@/types/core/infrastructure/queue';

export class WorkshopImpl extends BuildingImpl implements Workshop {
  public readonly id: BuildingId = 'workshop';
  public readonly recruitQueue: WorkshopRecruitQueueImpl;

  private constructor(workshop: Workshop) {
    super(workshop);

    this.recruitQueue = WorkshopRecruitQueueImpl.create(workshop.recruitQueue);
  }

  public hasRecruitOrder(id: InfrastructureQueueOrderId) {
    return this.recruitQueue.orders.some((order) => order.id === id);
  }

  public static create(workshop: Workshop) {
    return new WorkshopImpl(workshop);
  }
}
