// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { StorageImpl } from './abstract';
import type { StorageId, Warehouse } from '@tsukilabs/nil-bindings';

export class WarehouseImpl extends StorageImpl implements Readonly<Warehouse> {
  public readonly id: StorageId = 'warehouse';

  private constructor(warehouse: Warehouse) {
    super(warehouse);
  }

  public static create(warehouse: Warehouse) {
    return new WarehouseImpl(warehouse);
  }
}
