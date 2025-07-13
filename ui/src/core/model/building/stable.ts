// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from './abstract';

export class StableImpl extends BuildingImpl implements Stable {
  public readonly id: BuildingId = 'stable';

  private constructor(stable: Stable) {
    super(stable);
  }

  public static create(stable: Stable) {
    return new StableImpl(stable);
  }
}
