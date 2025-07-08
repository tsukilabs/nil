// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { PrefectureBuildOrderImpl } from './build-order';

export class PrefectureBuildQueueImpl implements PrefectureBuildQueue {
  public readonly orders: readonly PrefectureBuildOrderImpl[];

  private constructor(queue: { orders: PrefectureBuildOrderImpl[] }) {
    this.orders = queue.orders;
  }

  public *[Symbol.iterator]() {
    yield* this.orders;
  }

  public first() {
    return this.orders.at(0);
  }

  public last() {
    return this.orders.at(-1);
  }

  get size() {
    return this.orders.length;
  }

  public static create(queue: PrefectureBuildQueue) {
    const orders = queue.orders.map((it) => PrefectureBuildOrderImpl.create(it));
    return new PrefectureBuildQueueImpl({ orders });
  }
}
