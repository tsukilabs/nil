// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { MineImpl } from './abstract';
import type { MineId, Sawmill } from '@/types/bindings';

export class SawmillImpl extends MineImpl implements Readonly<Sawmill> {
  public readonly id: MineId = 'sawmill';

  private constructor(sawmill: Sawmill) {
    super(sawmill);
  }

  public static create(sawmill: Sawmill) {
    return new SawmillImpl(sawmill);
  }
}
