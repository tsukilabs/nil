// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ResourcesImpl } from '@/core/model/resources';

export class ManeuverHaulImpl implements ManeuverHaul {
  public readonly ruler: Ruler;
  public readonly resources: ResourcesImpl;

  private constructor(haul: ManeuverHaul) {
    this.ruler = haul.ruler;
    this.resources = ResourcesImpl.create(haul.resources);
  }

  public static create(haul: ManeuverHaul) {
    if (haul instanceof ManeuverHaulImpl) {
      return haul;
    }

    return new ManeuverHaulImpl(haul);
  }
}
