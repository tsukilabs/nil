// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Coord } from '@/types/core/continent';
import type { Resources } from '@/types/core/resources';
import type { Squad } from '@/types/core/military/squad';
import type { AcademyUnitId } from '@/types/core/military/unit';
import type { Building } from '@/types/core/infrastructure/building';
import type { ArmyAcademyPersonnel } from '@/types/core/military/army';
import type { InfrastructureRequirements } from '@/types/core/infrastructure';
import type { InfrastructureQueueOrder } from '@/types/core/infrastructure/queue';

export interface Academy extends Building {
  readonly recruitQueue: AcademyRecruitQueue;
}

export interface AcademyRecruitQueue {
  readonly orders: readonly AcademyRecruitOrder[];
}

export interface AcademyRecruitOrder extends InfrastructureQueueOrder {
  readonly squad: Squad;
}

export interface AcademyRecruitOrderRequest {
  readonly coord: Coord;
  readonly unit: AcademyUnitId;
  readonly chunks: number;
}

export type AcademyRecruitCatalog = {
  readonly [U in keyof ArmyAcademyPersonnel]: AcademyRecruitCatalogEntry;
};

export type AcademyRecruitCatalogEntry =
  | AcademyRecruitCatalogEntryAvailable
  | AcademyRecruitCatalogEntryUnmet;

export interface AcademyRecruitCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: AcademyRecruitCatalogRecipe;
}

export interface AcademyRecruitCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

export interface AcademyRecruitCatalogRecipe {
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
