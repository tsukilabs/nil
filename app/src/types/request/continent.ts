// Copyright (C) Call of Nil contrireadonlyors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';
import type { Coord } from '@/types/core/continent';

export interface GetContinentSizeRequest {
  readonly world: WorldId;
}

export interface GetPublicFieldRequest {
  readonly world: WorldId;
  readonly coord: Coord;
}

export interface GetPublicFieldsRequest {
  readonly world: WorldId;
  readonly coords: readonly Coord[];
}
