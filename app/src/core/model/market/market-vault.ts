// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PartialNullish } from "@tb-dev/utils";
import type { DeepReadonly } from "es-toolkit/types";
import { ResourcesImpl } from "@/core/model/resources";
import type { MarketVault, Resources } from "@tsukilabs/nil-bindings";

export class MarketVaultImpl implements DeepReadonly<MarketVault> {
  public readonly resources: ResourcesImpl;

  private constructor(vault: MarketVault) {
    this.resources = ResourcesImpl.create(vault.resources);
  }

  public hasResources(resources: PartialNullish<Resources>) {
    return this.resources.has(resources);
  }

  get food() {
    return this.resources.food;
  }

  get iron() {
    return this.resources.iron;
  }

  get stone() {
    return this.resources.stone;
  }

  get wood() {
    return this.resources.wood;
  }

  public static create(vault: MarketVault) {
    if (vault instanceof MarketVaultImpl) {
      return vault;
    }

    return new MarketVaultImpl(vault);
  }
}
