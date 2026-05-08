// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { MineImpl } from './abstract';
import type { Farm, MineId } from '@tsukilabs/nil-bindings';

export class FarmImpl extends MineImpl implements Readonly<Farm> {
  public readonly id: MineId = 'farm';

  private constructor(farm: Farm) {
    super(farm);
  }

  public static create(farm: Farm) {
    return new FarmImpl(farm);
  }
}
