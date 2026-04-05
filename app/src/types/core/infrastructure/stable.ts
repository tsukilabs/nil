// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Coord } from '@/types/core/continent';
import type { Resources } from '@/types/core/resources';
import type { Squad } from '@/types/core/military/squad';
import type { StableUnitId } from '@/types/core/military/unit';
import type { Building } from '@/types/core/infrastructure/building';
import type { ArmyStablePersonnel } from '@/types/core/military/army';
import type { InfrastructureRequirements } from '@/types/core/infrastructure';
import type { InfrastructureQueueOrder } from '@/types/core/infrastructure/queue';

export interface Stable extends Building {
  readonly recruitQueue: StableRecruitQueue;
}

export interface StableRecruitQueue {
  readonly orders: readonly StableRecruitOrder[];
}

export interface StableRecruitOrder extends InfrastructureQueueOrder {
  readonly squad: Squad;
}

export interface StableRecruitOrderRequest {
  readonly coord: Coord;
  readonly unit: StableUnitId;
  readonly chunks: number;
}

export type StableRecruitCatalog = {
  readonly [U in keyof ArmyStablePersonnel]: StableRecruitCatalogEntry;
};

export type StableRecruitCatalogEntry =
  | StableRecruitCatalogEntryAvailable
  | StableRecruitCatalogEntryUnmet;

export interface StableRecruitCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: StableRecruitCatalogRecipe;
}

export interface StableRecruitCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

export interface StableRecruitCatalogRecipe {
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
