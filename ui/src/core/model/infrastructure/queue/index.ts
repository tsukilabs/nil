// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export abstract class InfrastructureQueueImpl<Order extends InfrastructureQueueOrder>
  implements InfrastructureQueue<Order>
{
  public readonly orders: readonly Order[];

  protected constructor(queue: InfrastructureQueue<Order>) {
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
}
