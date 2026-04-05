// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Coord } from '@/types/core/continent';
import type { Resources } from '@/types/core/resources';
import type { Squad } from '@/types/core/military/squad';
import type { WorkshopUnitId } from '@/types/core/military/unit';
import type { Building } from '@/types/core/infrastructure/building';
import type { ArmyWorkshopPersonnel } from '@/types/core/military/army';
import type { InfrastructureQueueOrder } from '@/types/core/infrastructure/queue';

export interface Workshop extends Building {
  readonly recruitQueue: WorkshopRecruitQueue;
}

export interface WorkshopRecruitQueue {
  readonly orders: readonly WorkshopRecruitOrder[];
}

export interface WorkshopRecruitOrder extends InfrastructureQueueOrder {
  readonly squad: Squad;
}

export interface WorkshopRecruitOrderRequest {
  readonly coord: Coord;
  readonly unit: WorkshopUnitId;
  readonly chunks: number;
}

export type WorkshopRecruitCatalog = {
  readonly [U in keyof ArmyWorkshopPersonnel]: WorkshopRecruitCatalogEntry;
};

export type WorkshopRecruitCatalogEntry =
  | WorkshopRecruitCatalogEntryAvailable
  | WorkshopRecruitCatalogEntryUnmet;

export interface WorkshopRecruitCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: WorkshopRecruitCatalogRecipe;
}

export interface WorkshopRecruitCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

export interface WorkshopRecruitCatalogRecipe {
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
