// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from './abstract';

export class AcademyImpl extends BuildingImpl implements Academy {
  public readonly id: BuildingId = 'academy';

  private constructor(academy: Academy) {
    super(academy);
  }

  public static create(academy: Academy) {
    return new AcademyImpl(academy);
  }
}
