// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface AddPrefectureBuildOrderRequest {
  readonly world: WorldId;
  readonly request: PrefectureBuildOrderRequest;
}

export interface CancelPrefectureBuildOrderRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface GetPrefectureBuildCatalogRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}
