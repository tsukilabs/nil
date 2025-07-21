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
  readonly state: PrefectureBuildOrderState;
}

type PrefectureBuildOrderId = string;

type PrefectureBuildOrderKind = 'construction' | 'demolition';

type PrefectureBuildOrderState = PrefectureBuildOrderStatePending | PrefectureBuildOrderStateDone;

interface PrefectureBuildOrderStatePending {
  readonly kind: 'pending';
  readonly workforce: number;
}

interface PrefectureBuildOrderStateDone {
  readonly kind: 'done';
}

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
