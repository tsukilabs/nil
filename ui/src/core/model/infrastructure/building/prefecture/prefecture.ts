// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from '../abstract';
import { PrefectureBuildQueueImpl } from './build-queue';

export class PrefectureImpl extends BuildingImpl implements Prefecture {
  public readonly id: BuildingId = 'prefecture';
  public readonly buildQueue: PrefectureBuildQueueImpl;

  private constructor(prefecture: Prefecture) {
    super(prefecture);

    this.buildQueue = PrefectureBuildQueueImpl.create(prefecture.buildQueue);
  }

  public hasBuildOrder(id: InfrastructureQueueOrderId) {
    return this.buildQueue.orders.some((order) => order.id === id);
  }

  public resolveBuildingLevel(building: BuildingImpl) {
    let level = building.level;
    const min = building.minLevel;
    const max = building.maxLevel;

    for (const order of this.buildQueue) {
      if (order.building === building.id) {
        switch (order.kind) {
          case 'construction': {
            level = Math.min(level + 1, max);
            break;
          }
          case 'demolition': {
            level = Math.max(level - 1, min);
            break;
          }
        }
      }
    }

    return level;
  }

  public static create(prefecture: Prefecture) {
    return new PrefectureImpl(prefecture);
  }
}
