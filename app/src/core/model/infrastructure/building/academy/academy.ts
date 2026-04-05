// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from '../abstract';
import { AcademyRecruitQueueImpl } from './recruit-queue';
import type { Academy } from '@/types/core/infrastructure/academy';
import type { BuildingId } from '@/types/core/infrastructure/building';
import type { InfrastructureQueueOrderId } from '@/types/core/infrastructure/queue';

export class AcademyImpl extends BuildingImpl implements Academy {
  public readonly id: BuildingId = 'academy';
  public readonly recruitQueue: AcademyRecruitQueueImpl;

  private constructor(academy: Academy) {
    super(academy);

    this.recruitQueue = AcademyRecruitQueueImpl.create(academy.recruitQueue);
  }

  public hasRecruitOrder(id: InfrastructureQueueOrderId) {
    return this.recruitQueue.orders.some((order) => order.id === id);
  }

  public static create(academy: Academy) {
    return new AcademyImpl(academy);
  }
}
