// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { MineImpl } from './abstract';

export class IronMineImpl extends MineImpl implements IronMine {
  public readonly id: MineId = 'iron-mine';

  private constructor(ironMine: IronMine) {
    super(ironMine);
  }

  public static create(ironMine: IronMine) {
    return new IronMineImpl(ironMine);
  }
}
