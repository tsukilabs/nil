// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { InfrastructureQueueOrderImpl } from '@/core/model/infrastructure/queue/queue-order';

export class PrefectureBuildOrderImpl extends InfrastructureQueueOrderImpl
  implements PrefectureBuildOrder
{
  public readonly kind: PrefectureBuildOrderKind;
  public readonly building: BuildingId;
  public readonly level: BuildingLevel;

  private constructor(order: PrefectureBuildOrder) {
    super(order);

    this.kind = order.kind;
    this.building = order.building;
    this.level = order.level;
  }

  public static create(order: PrefectureBuildOrder) {
    return new PrefectureBuildOrderImpl(order);
  }
}
