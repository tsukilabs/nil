// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class OverallStorageCapacityImpl implements OverallStorageCapacity {
  public readonly silo: number;
  public readonly warehouse: number;

  private constructor(capacity: OverallStorageCapacity) {
    this.silo = capacity.silo;
    this.warehouse = capacity.warehouse;
  }

  public static create(capacity: OverallStorageCapacity) {
    return new OverallStorageCapacityImpl(capacity);
  }
}
