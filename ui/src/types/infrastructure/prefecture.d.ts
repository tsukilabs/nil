// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Prefecture extends Building {
  readonly buildQueue: PrefectureBuildQueue;
}

interface PrefectureBuildQueue {
  readonly orders: readonly PrefectureBuildOrder[];
}

interface PrefectureBuildOrder {
  readonly id: PrefectureBuildOrderId;
  readonly kind: PrefectureBuildOrderKind;
  readonly building: BuildingId;
  readonly level: BuildingLevel;
  readonly resources: Resources;
  readonly workforce: number;
  readonly status: PrefectureBuildOrderStatus;
}

type PrefectureBuildOrderId = string;

type PrefectureBuildOrderKind = 'construction' | 'demolition';

type PrefectureBuildOrderStatus =
  | PrefectureBuildOrderStatusDone
  | PrefectureBuildOrderStatusPending;

interface PrefectureBuildOrderStatusDone {
  readonly kind: 'done';
}

interface PrefectureBuildOrderStatusPending {
  readonly kind: 'pending';
  readonly workforce: number;
}

interface PrefectureBuildOrderOptions {
  readonly coord: Coord;
  readonly building: BuildingId;
  readonly kind: PrefectureBuildOrderKind;
}

interface PrefectureCatalogRecipe {
  readonly level: BuildingLevel;
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}

type PrefectureCatalog = {
  readonly [B in keyof Infrastructure]: PrefectureCatalogEntry;
};

type PrefectureCatalogEntry =
  | PrefectureCatalogEntryAvailable
  | PrefectureCatalogEntryMaxed
  | PrefectureCatalogEntryUnmet;

interface PrefectureCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: PrefectureCatalogRecipe;
}

interface PrefectureCatalogEntryMaxed {
  readonly kind: 'maxed';
}

interface PrefectureCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}
