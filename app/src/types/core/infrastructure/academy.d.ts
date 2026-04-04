// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Academy extends Building {
  readonly recruitQueue: AcademyRecruitQueue;
}

interface AcademyRecruitQueue {
  readonly orders: readonly AcademyRecruitOrder[];
}

interface AcademyRecruitOrder extends InfrastructureQueueOrder {
  readonly squad: Squad;
}

interface AcademyRecruitOrderRequest {
  readonly coord: Coord;
  readonly unit: AcademyUnitId;
  readonly chunks: number;
}

type AcademyRecruitCatalog = {
  readonly [U in keyof ArmyAcademyPersonnel]: AcademyRecruitCatalogEntry;
};

type AcademyRecruitCatalogEntry =
  | AcademyRecruitCatalogEntryAvailable
  | AcademyRecruitCatalogEntryUnmet;

interface AcademyRecruitCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: AcademyRecruitCatalogRecipe;
}

interface AcademyRecruitCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

interface AcademyRecruitCatalogRecipe {
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
