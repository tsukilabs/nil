// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { DeepReadonly } from "es-toolkit/types";
import type { Capital } from "@tsukilabs/nil-bindings";
import { PublicCapitalImpl } from "@/core/model/capital/public-capital";

export class CapitalImpl extends PublicCapitalImpl implements DeepReadonly<Capital> {
  private constructor(capital: Capital) {
    super(capital);
  }

  public static override create(capital: Capital) {
    if (capital instanceof CapitalImpl) {
      return capital;
    }

    return new CapitalImpl(capital);
  }
}
