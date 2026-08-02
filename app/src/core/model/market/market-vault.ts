// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { DeepReadonly } from "es-toolkit/types";
import { ResourcesImpl } from "@/core/model/resources";
import type { MarketVault } from "@tsukilabs/nil-bindings";

export class MarketVaultImpl implements DeepReadonly<MarketVault> {
  public readonly resources: ResourcesImpl;

  private constructor(vault: MarketVault) {
    this.resources = ResourcesImpl.create(vault.resources);
  }

  public static create(vault: MarketVault) {
    if (vault instanceof MarketVaultImpl) {
      return vault;
    }

    return new MarketVaultImpl(vault);
  }
}
