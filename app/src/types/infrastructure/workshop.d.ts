// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Workshop extends Building {
  readonly recruitQueue: WorkshopRecruitQueue;
}

interface WorkshopRecruitQueue {
  readonly orders: readonly WorkshopRecruitOrder[];
}

interface WorkshopRecruitOrder extends InfrastructureQueueOrder {
  readonly squad: Squad;
}

interface WorkshopRecruitOrderRequest {
  readonly coord: Coord;
  readonly unit: WorkshopUnitId;
  readonly chunks: number;
}

type WorkshopRecruitCatalog = {
  readonly [U in keyof ArmyWorkshopPersonnel]: WorkshopRecruitCatalogEntry;
};

type WorkshopRecruitCatalogEntry =
  | WorkshopRecruitCatalogEntryAvailable
  | WorkshopRecruitCatalogEntryUnmet;

interface WorkshopRecruitCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: WorkshopRecruitCatalogRecipe;
}

interface WorkshopRecruitCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

interface WorkshopRecruitCatalogRecipe {
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
