// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from './abstract';

export class WallImpl extends BuildingImpl implements Wall {
  public readonly id: BuildingId = 'wall';

  private constructor(wall: Wall) {
    super(wall);
  }

  public static create(wall: Wall) {
    return new WallImpl(wall);
  }
}
