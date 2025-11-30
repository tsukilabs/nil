// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Prefecture extends Building {
  readonly buildQueue: PrefectureBuildQueue;
}

interface PrefectureBuildQueue {
  readonly orders: readonly PrefectureBuildOrder[];
}

interface PrefectureBuildOrder extends InfrastructureQueueOrder {
  readonly kind: PrefectureBuildOrderKind;
  readonly building: BuildingId;
  readonly level: BuildingLevel;
}

type PrefectureBuildOrderKind = 'construction' | 'demolition';

interface PrefectureBuildOrderRequest {
  readonly coord: Coord;
  readonly building: BuildingId;
  readonly kind: PrefectureBuildOrderKind;
}

type PrefectureBuildCatalog = {
  readonly [B in keyof Infrastructure]: PrefectureBuildCatalogEntry;
};

type PrefectureBuildCatalogEntry =
  | PrefectureBuildCatalogEntryAvailable
  | PrefectureBuildCatalogEntryMaxed
  | PrefectureBuildCatalogEntryUnmet;

interface PrefectureBuildCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: PrefectureBuildCatalogRecipe;
}

interface PrefectureBuildCatalogEntryMaxed {
  readonly kind: 'maxed';
}

interface PrefectureBuildCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

interface PrefectureBuildCatalogRecipe {
  readonly level: BuildingLevel;
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
