// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from '../abstract';
import { PrefectureBuildQueueImpl } from './queue';

export class PrefectureImpl extends BuildingImpl implements Prefecture {
  public readonly id: BuildingId = 'prefecture';
  public readonly buildQueue: PrefectureBuildQueueImpl;

  private constructor(prefecture: Building & { buildQueue: PrefectureBuildQueueImpl }) {
    super(prefecture);
    this.buildQueue = prefecture.buildQueue;
  }

  public hasBuildOrder(id: PrefectureBuildOrderId) {
    return this.buildQueue.orders.some((order) => order.id === id);
  }

  public resolveBuildingLevel(building: BuildingId, level: BuildingLevel) {
    const { stats } = NIL.world.refs();
    const min = stats.value?.getBuildingMinLevel(building) ?? 0;
    const max = stats.value?.getBuildingMaxLevel(building) ?? 255;

    for (const order of this.buildQueue) {
      if (order.building === building) {
        switch (order.kind) {
          case 'construction': {
            level = Math.max(level + 1, max);
            break;
          }
          case 'demolition': {
            level = Math.min(level - 1, min);
            break;
          }
        }
      }
    }

    return level;
  }

  public static create(prefecture: Prefecture) {
    const buildQueue = PrefectureBuildQueueImpl.create(prefecture.buildQueue);
    return new PrefectureImpl({
      level: prefecture.level,
      enabled: prefecture.enabled,
      buildQueue,
    });
  }
}
