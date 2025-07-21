// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class PrefectureBuildOrderImpl implements PrefectureBuildOrder {
  public readonly id: string;
  public readonly kind: PrefectureBuildOrderKind;
  public readonly building: BuildingId;
  public readonly level: number;
  public readonly resources: Resources;
  public readonly workforce: number;
  public readonly state: PrefectureBuildOrderState;

  private constructor(order: PrefectureBuildOrder) {
    this.id = order.id;
    this.kind = order.kind;
    this.building = order.building;
    this.level = order.level;
    this.resources = order.resources;
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

  public static create(order: PrefectureBuildOrder) {
    return new PrefectureBuildOrderImpl(order);
  }
}
