// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { DeepReadonly } from "es-toolkit/types";
import { CoordImpl } from "@/core/model/continent/coord";
import type { PublicCapital } from "@tsukilabs/nil-bindings";

export class PublicCapitalImpl implements DeepReadonly<PublicCapital> {
  public readonly coord: CoordImpl | null;

  protected constructor(capital: PublicCapital) {
    this.coord = capital.coord ? CoordImpl.create(capital.coord) : null;
  }

  public static create(capital: PublicCapital) {
    if (capital instanceof PublicCapitalImpl) {
      return capital;
    }

    return new PublicCapitalImpl(capital);
  }
}
