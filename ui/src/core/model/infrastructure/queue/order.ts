// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ResourcesImpl } from '@/core/model/resources';

export abstract class InfrastructureQueueOrderImpl implements InfrastructureQueueOrder {
  public readonly id: string;
  public readonly resources: ResourcesImpl;
  public readonly workforce: number;
  public readonly state: InfrastructureQueueOrderState;

  protected constructor(order: InfrastructureQueueOrder) {
    this.id = order.id;
    this.resources = ResourcesImpl.create(order.resources);
    this.workforce = order.workforce;
    this.state = order.state;
  }

  public isDone() {
    return this.state.kind === 'done';
  }

  public isPending() {
    return this.state.kind === 'pending';
  }

  public getPendingWorkforce() {
    switch (this.state.kind) {
      case 'pending':
        return this.state.workforce;
      default:
        return 0;
    }
  }
}
