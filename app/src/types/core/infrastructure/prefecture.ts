// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Coord } from '@/types/core/continent';
import type { Resources } from '@/types/core/resources';
import type { InfrastructureQueueOrder } from '@/types/core/infrastructure/queue';
import type { Infrastructure, InfrastructureRequirements } from '@/types/core/infrastructure';
import type { Building, BuildingId, BuildingLevel } from '@/types/core/infrastructure/building';

export interface Prefecture extends Building {
  readonly buildQueue: PrefectureBuildQueue;
}

export interface PrefectureBuildQueue {
  readonly orders: readonly PrefectureBuildOrder[];
}

export interface PrefectureBuildOrder extends InfrastructureQueueOrder {
  readonly kind: PrefectureBuildOrderKind;
  readonly building: BuildingId;
  readonly level: BuildingLevel;
}

export type PrefectureBuildOrderKind = 'construction' | 'demolition';

export interface PrefectureBuildOrderRequest {
  readonly coord: Coord;
  readonly building: BuildingId;
  readonly kind: PrefectureBuildOrderKind;
}

export type PrefectureBuildCatalog = {
  readonly [B in keyof Infrastructure]: PrefectureBuildCatalogEntry;
};

export type PrefectureBuildCatalogEntry =
  | PrefectureBuildCatalogEntryAvailable
  | PrefectureBuildCatalogEntryMaxed
  | PrefectureBuildCatalogEntryUnmet;

export interface PrefectureBuildCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: PrefectureBuildCatalogRecipe;
}

export interface PrefectureBuildCatalogEntryMaxed {
  readonly kind: 'maxed';
}

export interface PrefectureBuildCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

export interface PrefectureBuildCatalogRecipe {
  readonly level: BuildingLevel;
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
