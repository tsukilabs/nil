// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { MineImpl } from './abstract';
import type { MineId } from '@/types/core/infrastructure/mine';
import type { Farm } from '@/types/core/infrastructure/building';

export class FarmImpl extends MineImpl implements Farm {
  public readonly id: MineId = 'farm';

  private constructor(farm: Farm) {
    super(farm);
  }

  public static create(farm: Farm) {
    return new FarmImpl(farm);
  }
}
