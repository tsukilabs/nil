// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from '../abstract';

export class WorkshopImpl extends BuildingImpl implements Workshop {
  public readonly id: BuildingId = 'workshop';

  private constructor(workshop: Workshop) {
    super(workshop);
  }

  public static create(workshop: Workshop) {
    return new WorkshopImpl(workshop);
  }
}
