// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';
import type { PrefectureBuildOrderRequest } from '@/types/core/infrastructure/prefecture';

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
