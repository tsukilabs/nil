// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { MineImpl } from './abstract';

export class QuarryImpl extends MineImpl implements Quarry {
  public readonly id: MineId = 'quarry';

  private constructor(quarry: Quarry) {
    super(quarry);
  }

  public static create(quarry: Quarry) {
    return new QuarryImpl(quarry);
  }
}
