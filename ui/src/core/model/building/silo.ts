// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { StorageImpl } from './abstract';

export class SiloImpl extends StorageImpl implements Silo {
  public readonly id: StorageId = 'silo';

  private constructor(silo: Silo) {
    super(silo);
  }

  public static create(silo: Silo) {
    return new SiloImpl(silo);
  }
}
