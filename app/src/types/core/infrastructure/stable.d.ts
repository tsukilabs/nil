// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Stable extends Building {
  readonly recruitQueue: StableRecruitQueue;
}

interface StableRecruitQueue {
  readonly orders: readonly StableRecruitOrder[];
}

interface StableRecruitOrder extends InfrastructureQueueOrder {
  readonly squad: Squad;
}

interface StableRecruitOrderRequest {
  readonly coord: Coord;
  readonly unit: StableUnitId;
  readonly chunks: number;
}

type StableRecruitCatalog = {
  readonly [U in keyof ArmyStablePersonnel]: StableRecruitCatalogEntry;
};

type StableRecruitCatalogEntry =
  | StableRecruitCatalogEntryAvailable
  | StableRecruitCatalogEntryUnmet;

interface StableRecruitCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: StableRecruitCatalogRecipe;
}

interface StableRecruitCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

interface StableRecruitCatalogRecipe {
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
